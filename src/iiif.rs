pub mod image;
pub mod info;
pub(crate) mod parse;

use std::num::NonZero;

#[derive(Debug, PartialEq)]
pub enum IiifRequest {
    Info {
        identifier: String,
    },
    Image {
        identifier: String,
        region: Region,
        size: Size,
        rotation: Rotation,
        quality: Quality,
        format: Format,
    },
}

impl IiifRequest {
    pub fn info<S: Into<String>>(identifier: S) -> Self {
        IiifRequest::Info { identifier: identifier.into() }
    }

    pub fn image<S: Into<String>>(
        identifier: S,
        region: Region,
        size: Size,
        rotation: Rotation,
        quality: Quality,
        format: Format,
    ) -> IiifRequest {
        IiifRequest::Image {
            identifier: identifier.into(),
            region,
            size,
            rotation,
            quality,
            format,
        }
    }
}

pub type Dimension = u32;

#[derive(Clone, Debug, PartialEq)]
pub enum Region {
    Full,
    Square,
    Absolute { x: Dimension, y: Dimension, width: Dimension, height: Dimension },
    Percentage { x: f32, y: f32, width: f32, height: f32 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    scale: Scale,
    upscale: bool,
}

impl Size {
    pub fn new(scale: Scale) -> Size {
        Size { upscale: false, scale }
    }

    pub fn upscaled(scale: Scale) -> Size {
        Size { upscale: true, scale }
    }

    pub fn scale(&self) -> Scale {
        self.scale
    }

    pub fn upscale(&self) -> bool {
        self.upscale
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scale {
    Max,
    Percentage(f32),
    Fixed { width: Option<NonZero<Dimension>>, height: Option<NonZero<Dimension>> },
    AspectPreserving { width: NonZero<Dimension>, height: NonZero<Dimension> },
}

impl Scale {
    pub fn fixed(width: Option<NonZero<Dimension>>, height: Option<NonZero<Dimension>>) -> Scale {
        Scale::Fixed { width, height }
    }

    pub fn fixed_height(height: Option<NonZero<Dimension>>) -> Scale {
        Scale::Fixed { width: None, height }
    }

    pub fn fixed_width(width: Option<NonZero<Dimension>>) -> Scale {
        Scale::Fixed { width, height: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotation {
    degrees: f32,
    mirror: bool,
}

impl Rotation {
    pub fn new(degrees: f32) -> Rotation {
        Rotation { mirror: false, degrees }
    }

    pub fn mirrored(degrees: f32) -> Rotation {
        Rotation { mirror: true, degrees }
    }

    pub fn degrees(&self) -> f32 {
        self.degrees
    }

    pub fn mirror(&self) -> bool {
        self.mirror
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Quality {
    Color,
    Gray,
    Bitonal,
    Default,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Format {
    Jpg,
    Tif,
    Png,
    Gif,
    Jp2,
    Pdf,
    Webp,
}

impl Format {
    pub fn mime(&self) -> &'static str {
        match self {
            Format::Jpg => "image/jpeg",
            Format::Tif => "image/tiff",
            Format::Png => "image/png",
            Format::Gif => "image/gif",
            Format::Jp2 => "image/jp2",
            Format::Pdf => "application/pdf",
            Format::Webp => "image/webp",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::iiif::Scale;

    #[test]
    fn decode_basic_info_request() {
        let request = "/abcd1234/info.json".parse();

        assert_eq!(request, Ok(IiifRequest::info("abcd1234")));
    }

    #[test]
    fn decode_basic_image_request() {
        let request = "/abcd1234/full/max/0/default.jpg".parse();

        assert_eq!(
            request,
            Ok(IiifRequest::image(
                "abcd1234",
                Region::Full,
                Size::new(Scale::Max),
                Rotation::new(0.0),
                Quality::Default,
                Format::Jpg,
            ))
        );
    }

    #[test]
    fn decode_encoded_image_request() {
        // Image API 3.0, s 9: to-encode = "/" / "?" / "#" / "[" / "]" / "@" / "%"
        let request = "/a%2F%3F%23%5B%5D%40%25z/full/max/0/default.jpg".parse();

        assert_eq!(
            request,
            Ok(IiifRequest::image(
                "a/?#[]@%z",
                Region::Full,
                Size::new(Scale::Max),
                Rotation::new(0.0),
                Quality::Default,
                Format::Jpg,
            ))
        );
    }
}
