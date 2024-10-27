use std::future::Future;

use super::ImageSource;
use crate::iiif::info::ImageInfo;

mod kaduceus;
pub use kaduceus::KaduceusImageReader;

pub trait ImageMetadataResolver {
    fn info<'a>(&'a self, location: ImageSource) -> Box<dyn Future<Output = ImageInfo> + 'a>;
}
