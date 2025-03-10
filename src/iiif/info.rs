use crate::iiif::Dimension;

#[allow(unused)]
pub struct ImageInfo {
    // @context: "http://iiif.io/api/image/3/context.json",
    // type: "ImageService3",
    // protocol: "http://iiif.io/api/image",
    // profile: "level0",
    /// The URI of the image, including scheme, server, etc.
    id: String,

    /// The width of the full image, in pixels.
    width: Dimension,

    /// The height of the full image, in pixels.
    height: Dimension,

    /// The maximum width the image can be scaled to, in pixels.
    max_width: Option<Dimension>,

    /// The maximum height the image can be scaled to, in pixels.
    max_height: Option<Dimension>,

    /// The maximum area the image can be scaled to, in pixels.
    max_area: Option<Dimension>,

    /// The preferred sizes (if any) for scaled versions of the image.
    sizes: Option<Vec<PreferredSize>>,

    /// The regions of the image that can be visually stitched together to create the full image.
    tiles: Option<Vec<Tile>>,

    /// The preferred format(s) for this the image.
    preferred_formats: Option<Vec<String>>,

    /// The license or rights statement that applies to the image.
    rights: Option<String>,
}

#[allow(unused)]
pub struct PreferredSize {
    // type: "Size",
    width: Dimension,
    height: Dimension,
}

#[allow(unused)]
pub struct Tile {
    // type: "Tile",
    scale_factors: Vec<u16>,
    width: Dimension,
    height: Option<Dimension>,
}
