mod locator;
pub use locator::{ ImageSourceResolver, LocalImageSourceResolver};

mod pipeline;
pub use pipeline::{ImagePipeline, ImagePipelineBuilder};

pub mod metadata;
pub use metadata::ImageMetadataResolver;
