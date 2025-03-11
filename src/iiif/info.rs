use crate::iiif::Dimension;

#[allow(unused)]
pub struct ImageInfo {
    // @context: "http://iiif.io/api/image/3/context.json",
    // type: "ImageService3",
    // protocol: "http://iiif.io/api/image",
    // profile: "level0",
    /// The width of the full image, in pixels.
    pub width: Dimension,

    /// The height of the full image, in pixels.
    pub height: Dimension,

    /// The maximum width the image can be scaled to, in pixels.
    pub max_width: Option<Dimension>,

    /// The maximum height the image can be scaled to, in pixels.
    pub max_height: Option<Dimension>,

    /// The maximum area the image can be scaled to, in pixels.
    pub max_area: Option<Dimension>,

    /// The preferred sizes (if any) for scaled versions of the image.
    pub sizes: Option<Vec<PreferredSize>>,

    /// The regions of the image that can be visually stitched together to create the full image.
    pub tiles: Option<Vec<Tile>>,

    /// The preferred format(s) for this the image.
    pub preferred_formats: Option<Vec<String>>,

    /// The license or rights statement that applies to the image.
    pub rights: Option<String>,
}

#[allow(unused)]
pub struct PreferredSize {
    // type: "Size",
    pub width: Dimension,
    pub height: Dimension,
}

#[allow(unused)]
pub struct Tile {
    // type: "Tile",
    pub scale_factors: Vec<u16>,
    pub width: Dimension,
    pub height: Option<Dimension>,
}
