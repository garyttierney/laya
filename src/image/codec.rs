use std::future::Future;
use std::pin::Pin;

use crate::storage::FileOrStream;

mod kaduceus;
pub use kaduceus::KaduceusImageReader;

use super::BoxedImage;

/// The [`ImageReader`] trait defines functionality for reading and decoding various image formats.
///
/// This trait provides an asynchronous mechanism for reading image data from a file or stream and
/// converting it into a usable representation. Implementations are expected to handle specific
/// image formats and return a boxed [`Image`](super::Image) object that can be used for further
/// processing.
///
/// # Example
///
/// ```rust
/// use laya::image::{BoxedImage, ImageReader};
/// use laya::storage::FileOrStream;
///
/// async fn decode_image(reader: impl ImageReader, file_stream: FileOrStream) {
///     let image = reader.read(None, file_stream).await;
///     let info = image.info();
///     println!("Image dimensions: {}x{}", info.width, info.height);
/// }
/// ```
pub trait ImageReader: Send + Sync {
    fn read<'a>(
        &'a self,
        name: Option<String>,
        location: FileOrStream,
    ) -> Pin<Box<dyn Future<Output = BoxedImage> + Send + 'a>>;
}

impl ImageReader for Box<dyn ImageReader> {
    fn read<'a>(
        &'a self,
        name: Option<String>,
        location: FileOrStream,
    ) -> Pin<Box<dyn Future<Output = BoxedImage> + Send + 'a>> {
        <dyn ImageReader>::read(self, name, location)
    }
}

impl<T: ImageReader> ImageReader for Box<T> {
    fn read<'a>(
        &'a self,
        name: Option<String>,
        location: FileOrStream,
    ) -> Pin<Box<dyn Future<Output = BoxedImage> + Send + 'a>> {
        T::read(self, name, location)
    }
}
