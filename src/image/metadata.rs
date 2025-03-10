use std::future::Future;

use crate::iiif::info::ImageInfo;
use crate::storage::FileOrStream;

mod kaduceus;
pub use kaduceus::KaduceusImageReader;

pub trait ImageMetadataResolver {
    fn info<'a>(&'a self, location: FileOrStream) -> Box<dyn Future<Output = ImageInfo> + 'a>;
}
