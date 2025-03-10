use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::NonZero;
use std::str::FromStr;

use super::IiifRequest;
use crate::http::IiifRequestError;
use crate::iiif::{Dimension, Format, Quality, Region, Rotation, Scale, Size};

const PERCENT_PREFIX: &str = "pct:";
const REGION_SELECTOR_COUNT: usize = 4;

impl FromStr for Region {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "full" => Ok(Region::Full),
            "square" => Ok(Region::Square),
            _ => {
                if let Some(input) = s.strip_prefix(PERCENT_PREFIX) {
                    let [x, y, width, height] = parse_region_selectors(input)?;
                    Ok(Region::Percentage { x, y, width, height })
                } else {
                    let [x, y, width, height] = parse_region_selectors(s)?;
                    Ok(Region::Absolute { x, y, width, height })
                }
            }
        }
    }
}

impl FromStr for IiifRequest {
    type Err = IiifRequestError;

    fn from_str(path: &str) -> Result<Self, Self::Err> {
        let mut segments = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .peekable();

        let identifier = segments
            .next()
            .ok_or(IiifRequestError::UriMissingElement("identifier"))
            .and_then(|input| {
                urlencoding::decode(input).map_err(|err| IiifRequestError::UriNotUtf8("identifier"))
            })?;

        let is_info_request = segments.peek().is_some_and(|s| *s == "info.json");
        if is_info_request {
            return Ok(IiifRequest::Info { identifier: identifier.into() });
        }

        let region = segments
            .next()
            .ok_or(IiifRequestError::UriMissingElement("region"))?
            .parse::<Region>()
            .map_err(IiifRequestError::from)?;

        let size = segments
            .next()
            .ok_or(IiifRequestError::UriMissingElement("size"))?
            .parse::<Size>()
            .map_err(IiifRequestError::from)?;

        let rotation = segments
            .next()
            .ok_or(IiifRequestError::UriMissingElement("rotation"))?
            .parse::<Rotation>()
            .map_err(IiifRequestError::from)?;

        let (quality, format) = segments
            .next()
            .ok_or(IiifRequestError::UriMissingElement("quality"))?
            .split_once('.')
            .ok_or(IiifRequestError::UriMissingElement("format"))?;

        let quality = quality
            .parse::<Quality>()
            .map_err(IiifRequestError::ParseError)?;

        let format = format
            .parse::<Format>()
            .map_err(IiifRequestError::ParseError)?;

        Ok(IiifRequest::image(identifier, region, size, rotation, quality, format))
    }
}

fn parse_region_selectors<T: SpatialSelector>(
    input: &str,
) -> Result<[T; REGION_SELECTOR_COUNT], ParseError> {
    let mut selectors = [T::default(); REGION_SELECTOR_COUNT];
    let mut split = input.split(",");

    for (index, item) in selectors.iter_mut().enumerate() {
        let s = split
            .next()
            .ok_or(ParseError::RegionSelectorCount(input.into()))?;

        *item = s
            .parse::<T>()
            .map_err(|_| ParseError::region_unparsable(s, index))
            .and_then(|value| T::validate(value, index))?;
    }

    match split.next() {
        Some(_) => Err(ParseError::RegionSelectorCount(input.into())),
        None => Ok(selectors),
    }
}

trait SpatialSelector: Copy + Default + FromStr + ToString {
    fn validate(value: Self, index: usize) -> Result<Self, ParseError>;
}

impl SpatialSelector for f32 {
    fn validate(value: Self, index: usize) -> Result<Self, ParseError> {
        // Image API 3.0, s 4.1: region parameters in percentages ... must be positive
        if value > 0.0 {
            Ok(value)
        } else {
            Err(ParseError::RegionPercentageOutOfBounds { input: value.to_string(), index })
        }
    }
}

impl SpatialSelector for u32 {
    fn validate(value: Self, index: usize) -> Result<Self, ParseError> {
        // Image API 3.0, s 4.1: If the requested region’s height or width is zero, [fail].
        if index <= 1 /* x or y */ || value > 0 {
            Ok(value)
        } else {
            Err(ParseError::RegionSelectorOutOfBounds { input: value.to_string(), index })
        }
    }
}

fn selector_name(index: usize) -> &'static str {
    match index {
        0 => "x",
        1 => "y",
        2 => "width",
        3 => "height",
        _ => panic!("Region selector index must be [0, 3]."),
    }
}

impl FromStr for Size {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let upscale = s.starts_with('^');
        let s = if upscale { &s[1..] } else { s };

        if s == "max" {
            return Ok(Size { upscale, scale: Scale::Max });
        } else if let Some(s) = s.strip_prefix(PERCENT_PREFIX) {
            return parse_scale_percent(s, upscale);
        }

        let preserve_ratio = s.starts_with('!');
        let s = if preserve_ratio { &s[1..] } else { s };

        if s == "," {
            return Err(ParseError::SizeMissingDimensions(s.into()));
        }

        let (width, height) = s
            .split_once(',')
            .ok_or(ParseError::SizeMissingComma(s.into()))?;

        let scale = if preserve_ratio {
            Scale::AspectPreserving {
                width: parse_scale_px(width)?,
                height: parse_scale_px(height)?,
            }
        } else {
            let width = if width.is_empty() {
                None
            } else {
                Some(parse_scale_px(width)?)
            };
            let height = if height.is_empty() {
                None
            } else {
                Some(parse_scale_px(height)?)
            };

            Scale::Fixed { width, height }
        };

        Ok(Size { upscale, scale })
    }
}

fn parse_scale_percent(s: &str, upscale: bool) -> Result<Size, ParseError> {
    let scale = s
        .parse::<f32>()
        .map_err(|_| ParseError::SizePercentageUnparsable(s.into()))?;

    // Image API 3.0, s 4: Size ... parameters in percentages ... must be positive
    // Image API 3.0, s 4.2: n must not be greater than 100 [unless explicitly upscaling].
    if scale > 0.0 && (scale <= 100.0 || upscale) {
        Ok(Size { upscale, scale: Scale::Percentage(scale) })
    } else {
        Err(ParseError::SizePercentageOutOfBounds(s.into()))
    }
}

fn parse_scale_px(input: &str) -> Result<NonZero<Dimension>, ParseError> {
    let dimension = input
        .parse::<u32>()
        .map_err(|_| ParseError::SizeDimensionUnparsable(input.into()))?;

    NonZero::new(dimension).ok_or(ParseError::SizeDimensionOutOfBounds(input.into()))
}

impl FromStr for Rotation {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mirror = s.starts_with('!');
        let rotation = if mirror { &s[1..] } else { s };

        // Accept a rotation of 0 as examples (e.g. s 9) show it should be supported despite the
        // spec containing the following contradiction:
        // Image API 3.0, s 4: the rotation parameter MUST be positive [as a float or integer]
        // Image API 3.0, s 4.3: The [rotation] ... may be any floating point number from 0 to 360

        rotation
            .parse::<f32>()
            .map_err(|_| ParseError::RotationAngleUnparsable(rotation.into()))
            .and_then(|degrees| match degrees {
                0f32..=360.0 => Ok(Rotation { mirror, degrees }),
                _ => Err(ParseError::RotationOutOfBounds(rotation.into())),
            })
    }
}

impl FromStr for Quality {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "color" => Ok(Quality::Color),
            "gray" => Ok(Quality::Gray),
            "bitonal" => Ok(Quality::Bitonal),
            "default" => Ok(Quality::Default),
            _ => Err(ParseError::UnrecognisedQuality(s.into())),
        }
    }
}

impl FromStr for Format {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "jpg" => Ok(Format::Jpg),
            "tif" => Ok(Format::Tif),
            "png" => Ok(Format::Png),
            "gif" => Ok(Format::Gif),
            "jp2" => Ok(Format::Jp2),
            "pdf" => Ok(Format::Pdf),
            "webp" => Ok(Format::Webp),
            _ => Err(ParseError::UnrecognisedFormat(s.into())),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// If a spatial selector (x, y, width, height) could not be parsed.
    RegionSelectorUnparsable { input: String, index: usize },

    /// If a pixel spatial selector (x, y, width, height) is out of bounds.
    RegionSelectorOutOfBounds { input: String, index: usize },

    /// If a percentage spatial selectors (x, y, width, height) is out of bounds.
    RegionPercentageOutOfBounds { input: String, index: usize },

    /// If a spatial selector was missing, or too many were provided.
    RegionSelectorCount(String),

    /// If a size parameter omits the comma between width,height (e.g. `19201080`).
    SizeMissingComma(String),

    /// If a size parameter omits both width and height (e.g. `,` or `^,`).
    SizeMissingDimensions(String),

    /// If a size parameter contains a width/height value that cannot be parsed as an integer.
    SizeDimensionUnparsable(String),

    /// If a size parameter contains a width/height percentage that cannot be parsed as a float.
    SizePercentageOutOfBounds(String),

    /// If a size parameter contains a width/height value that is out of bounds.
    SizeDimensionOutOfBounds(String),

    /// If a size parameter contains a width/height percentage that is out of bounds.
    SizePercentageUnparsable(String),

    /// If the degrees to rotate by could not be parsed as a float.
    RotationAngleUnparsable(String),

    /// If the degrees to rotate by is out of bounds.
    RotationOutOfBounds(String),

    /// If the requested quality is unrecognised.
    UnrecognisedQuality(String),

    /// If the requested format is unrecognised.
    UnrecognisedFormat(String),
}

impl ParseError {
    fn region_unparsable<S: Into<String>>(input: S, index: usize) -> ParseError {
        ParseError::RegionSelectorUnparsable { input: input.into(), index }
    }
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::RegionSelectorUnparsable { input, index } => write!(
                f,
                "Region {} '{input}' could not be parsed (expected integer for standard requests, \
                 float for pct: requests).",
                selector_name(*index)
            ),
            ParseError::RegionSelectorOutOfBounds { input, index } => write!(
                f,
                "Region {} '{input}' out of bounds: width and height must be greater than 0.",
                selector_name(*index)
            ),
            ParseError::RegionPercentageOutOfBounds { input, index } => write!(
                f,
                "Region {} percentage '{input}' out of bounds: must be greater than 0.",
                selector_name(*index)
            ),
            ParseError::RegionSelectorCount(s) => {
                write!(f, "Region must include exactly 4 selectors x,y,w,h (received '{s}').")
            }
            ParseError::SizeMissingComma(s) => {
                write!(f, "Size must include a comma to separate dimensions (received '{s}').")
            }
            ParseError::SizeMissingDimensions(s) => {
                write!(f, "Size must include at least one dimension (received '{s}').")
            }
            ParseError::SizeDimensionUnparsable(s) => {
                write!(f, "Size dimension '{s}' could not be parsed (expected integer).")
            }
            ParseError::SizePercentageUnparsable(s) => {
                write!(f, "Size percentage '{s}' could not be parsed (expected integer or float).")
            }
            ParseError::SizeDimensionOutOfBounds(s) => {
                write!(f, "Size dimension '{s}' out of bounds: must be at least 1.")
            }
            ParseError::SizePercentageOutOfBounds(s) => write!(
                f,
                "Size percentage '{s}' out of bounds: must be greater than 0 if upscaling \
                 is requested, or (0.0, 100.0] otherwise."
            ),
            ParseError::RotationAngleUnparsable(s) => {
                write!(f, "Rotation '{s}' could not be parsed (expected integer or float).")
            }
            ParseError::RotationOutOfBounds(s) => {
                write!(f, "Rotation must be [0.0, 360.0], but was '{s}'.")
            }
            ParseError::UnrecognisedQuality(s) => write!(f, "Unrecognised quality '{s}'."),
            ParseError::UnrecognisedFormat(s) => write!(f, "Unrecognised format '{s}'."),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn full_region() {
        let result = "full".parse::<Region>();
        assert_eq!(result, Ok(Region::Full));
    }

    #[test]
    fn square_region() {
        let result = "square".parse::<Region>();
        assert_eq!(result, Ok(Region::Square));
    }

    #[test]
    fn percent_region() {
        let result = "pct:5,5,90,90".parse::<Region>();
        assert_eq!(result, Ok(Region::Percentage { x: 5.0, y: 5.0, width: 90.0, height: 90.0 }));

        let result = "pct:5.1,5.2,90.3,90.4".parse::<Region>();
        assert_eq!(result, Ok(Region::Percentage { x: 5.1, y: 5.2, width: 90.3, height: 90.4 }));
    }

    #[test]
    fn absolute_region() {
        let result = "5,5,50,50".parse::<Region>();
        assert_eq!(result, Ok(Region::Absolute { x: 5, y: 5, width: 50, height: 50 }));
    }

    #[test]
    fn region_selector_count_err() {
        let input = "5,5,1";
        assert_eq!(input.parse::<Region>(), Err(ParseError::RegionSelectorCount(input.into())));

        let input = "5,5,50,50,";
        assert_eq!(input.parse::<Region>(), Err(ParseError::RegionSelectorCount(input.into())));

        let input = "5,5,50,50,0";
        assert_eq!(input.parse::<Region>(), Err(ParseError::RegionSelectorCount(input.into())));
    }

    #[test]
    fn region_selector_unparsable_err() {
        let input = "abcdefg";
        assert_eq!(
            input.parse::<Region>(),
            Err(ParseError::RegionSelectorUnparsable { input: input.into(), index: 0 })
        );

        let result = "5,5,c,5".parse::<Region>();
        assert_eq!(
            result,
            Err(ParseError::RegionSelectorUnparsable { input: "c".into(), index: 2 })
        );

        let result = "5,5,5,".parse::<Region>();
        assert_eq!(
            result,
            Err(ParseError::RegionSelectorUnparsable { input: "".into(), index: 3 })
        );
    }

    #[test]
    fn region_selector_out_of_bounds_err() {
        let result = "5,5,0,90".parse::<Region>();
        assert_eq!(
            result,
            Err(ParseError::RegionSelectorOutOfBounds { input: "0".into(), index: 2 })
        );

        let result = "5,5,90,0".parse::<Region>();
        assert_eq!(
            result,
            Err(ParseError::RegionSelectorOutOfBounds { input: "0".into(), index: 3 })
        );
    }

    #[test]
    fn size_max() {
        let result = "max".parse::<Size>();
        assert_eq!(result, Ok(Size::new(Scale::Max)));
    }

    #[test]
    fn size_upscale() {
        let result = "^max".parse::<Size>();
        assert_eq!(result, Ok(Size::upscaled(Scale::Max)));

        let result = "^3840,".parse::<Size>();
        assert_eq!(result, Ok(Size::upscaled(Scale::fixed_width(NonZero::new(3840)))));

        let result = "^,2160".parse::<Size>();
        assert_eq!(result, Ok(Size::upscaled(Scale::fixed_height(NonZero::new(2160)))));

        let result = "^pct:200".parse::<Size>();
        assert_eq!(result, Ok(Size::upscaled(Scale::Percentage(200.0))));

        let result = "^3840,2160".parse::<Size>();
        assert_eq!(
            result,
            Ok(Size::upscaled(Scale::fixed(NonZero::new(3840), NonZero::new(2160))))
        );
    }

    #[test]
    fn size_scale_one_dimension() {
        let result = "960,".parse::<Size>();
        assert_eq!(result, Ok(Size::new(Scale::fixed_width(NonZero::new(960)))));

        let result = ",540".parse::<Size>();
        assert_eq!(result, Ok(Size::new(Scale::fixed_height(NonZero::new(540)))));
    }

    #[test]
    fn size_scale_percent() {
        let result = "pct:50".parse::<Size>();
        assert_eq!(result, Ok(Size::new(Scale::Percentage(50.0))));
    }

    #[test]
    fn size_scale_exact() {
        let result = "960,540".parse::<Size>();
        assert_eq!(result, Ok(Size::new(Scale::fixed(NonZero::new(960), NonZero::new(540)))));
    }

    #[test]
    fn size_scale_preserve_aspect() {
        let result = "!960,720".parse::<Size>();
        assert_eq!(
            result,
            Ok(Size::new(Scale::AspectPreserving {
                width: NonZero::new(960).unwrap(),
                height: NonZero::new(720).unwrap(),
            }))
        );

        let result = "^!3840,1620".parse::<Size>();
        assert_eq!(
            result,
            Ok(Size::upscaled(Scale::AspectPreserving {
                width: NonZero::new(3840).unwrap(),
                height: NonZero::new(1620).unwrap(),
            }))
        );
    }

    #[test]
    fn size_missing_comma_err() {
        let result = "secret".parse::<Size>();
        assert_eq!(result, Err(ParseError::SizeMissingComma("secret".into())));
    }

    #[test]
    fn size_px_unparsable_err() {
        let result = "1.1,1".parse::<Size>();
        assert_eq!(result, Err(ParseError::SizeDimensionUnparsable("1.1".into())));
    }

    #[test]
    fn size_px_out_of_bounds_err() {
        let result = "0,1".parse::<Size>();
        assert_eq!(result, Err(ParseError::SizeDimensionOutOfBounds("0".into())));

        let result = "1,0".parse::<Size>();
        assert_eq!(result, Err(ParseError::SizeDimensionOutOfBounds("0".into())));
    }

    #[test]
    fn size_percent_unparsable_err() {
        let result = "pct:yes".parse::<Size>();
        assert_eq!(result, Err(ParseError::SizePercentageUnparsable("yes".into())));
    }

    #[test]
    fn size_percent_out_of_bounds_err() {
        let result = "pct:0".parse::<Size>();
        assert_eq!(result, Err(ParseError::SizePercentageOutOfBounds("0".into())));

        let result = "pct:90125".parse::<Size>();
        assert_eq!(result, Err(ParseError::SizePercentageOutOfBounds("90125".into())));
    }

    #[test]
    fn rotation() {
        let result = "180".parse::<Rotation>();
        assert_eq!(result, Ok(Rotation::new(180.0)));

        let result = "180.42".parse::<Rotation>();
        assert_eq!(result, Ok(Rotation::new(180.42)));
    }

    #[test]
    fn mirrored_rotation() {
        let result = "!180".parse::<Rotation>();
        assert_eq!(result, Ok(Rotation::mirrored(180.0)));
    }

    #[test]
    fn rotation_out_of_bounds_err() {
        let result = "-1".parse::<Rotation>();
        assert_eq!(result, Err(ParseError::RotationOutOfBounds("-1".into())));
    }

    #[test]
    fn rotation_unparsable_err() {
        let result = "TRaFoaMP20230922".parse::<Rotation>();
        assert_eq!(result, Err(ParseError::RotationAngleUnparsable("TRaFoaMP20230922".into())));
    }

    #[test]
    fn quality() {
        let result = "color".parse::<Quality>();
        assert_eq!(result, Ok(Quality::Color));
    }

    #[test]
    fn unrecognised_quality_err() {
        let result = "TRaFoaMP20230922".parse::<Quality>();
        assert_eq!(result, Err(ParseError::UnrecognisedQuality("TRaFoaMP20230922".into())));
    }

    #[test]
    fn unrecognised_format_err() {
        let result = "TRaFoaMP20230922".parse::<Format>();
        assert_eq!(result, Err(ParseError::UnrecognisedFormat("TRaFoaMP20230922".into())));
    }
}
