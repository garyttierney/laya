mod info;
mod image;
pub(crate) mod parse;

use std::num::NonZero;

pub struct ImageRequest {
    identifier: String,
    region: Region,
    size: Size,
    rotation: Rotation,
    quality: Quality,
    format: Format,
}

pub type Dimension = u32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Region {
    Full,
    Square,
    Absolute { x: Dimension, y: Dimension, width: Dimension, height: Dimension },
    Percentage { x: f32, y: f32, width: f32, height: f32 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    upscale: bool,
    scale: Scale,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scale {
    Maximum,
    Percentage(f32),
    Fixed { width: Option<NonZero<Dimension>>, height: Option<NonZero<Dimension>> },
    AspectPreserving { width: NonZero<Dimension>, height: NonZero<Dimension> },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotation {
    pub mirror: bool,
    pub degrees: f32,
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
