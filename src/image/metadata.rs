use std::future::Future;
use std::pin::Pin;

use crate::iiif::info::ImageInfo;
use crate::storage::FileOrStream;

mod kaduceus;
pub use kaduceus::KaduceusImageReader;

use super::{BoxedImage, Image};

pub trait ImageReader {
    fn read<'a>(
        &'a self,
        location: FileOrStream,
    ) -> Pin<Box<dyn Future<Output = BoxedImage> + Send + 'a>>;
}
