use std::future::Future;
use std::pin::Pin;

use crate::storage::FileOrStream;

mod kaduceus;
pub use kaduceus::KaduceusImageReader;

use super::BoxedImage;

pub trait ImageReader: Send + Sync {
    fn read<'a>(
        &'a self,
        location: FileOrStream,
    ) -> Pin<Box<dyn Future<Output = BoxedImage> + Send + 'a>>;
}

impl ImageReader for Box<dyn ImageReader> {
    fn read<'a>(
        &'a self,
        location: FileOrStream,
    ) -> Pin<Box<dyn Future<Output = BoxedImage> + Send + 'a>> {
        <dyn ImageReader>::read(self, location)
    }
}

impl<T: ImageReader> ImageReader for Box<T> {
    fn read<'a>(
        &'a self,
        location: FileOrStream,
    ) -> Pin<Box<dyn Future<Output = BoxedImage> + Send + 'a>> {
        T::read(self, location)
    }
}
