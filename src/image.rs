use std::path::Path;

use kaduceus::{KakaduContext, KakaduImageReader};
use tokio::io::AsyncRead;

use crate::iiif::info::ImageInfo;

mod locator;
pub use locator::{ImageSource, ImageSourceResolver, LocalImageSourceResolver};

mod pipeline;
pub use pipeline::{ImagePipeline, ImagePipelineBuilder};

pub mod metadata;
pub use metadata::ImageMetadataResolver;
