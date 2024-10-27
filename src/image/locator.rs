use std::path::Path;

use tokio::io::{AsyncRead, AsyncSeek};

pub trait AsyncReadSeekable: AsyncRead + AsyncSeek {}
impl<R: AsyncRead + AsyncSeek> AsyncReadSeekable for R {}

pub type FileStreamFactory = Box<dyn FnOnce(&Path) -> Box<dyn AsyncRead>>;

pub struct FileStream {
    path: Box<Path>,
    stream_factory: FileStreamFactory,
}

/// An image source.
///
/// This enum represents the different ways an image can be loaded.
///
/// The reason `File` and `Stream` are separate despite `AsyncRead` being
/// able to represent a file is so decoders can decide to make optimizations
/// based on direct filesystem access, such as using memory mapped files or
/// DMA.
pub enum ImageSource {
    /// An image loaded from a file on the filesystem.
    File(FileStream),

    /// An image loaded from a stream.
    Stream(Box<dyn AsyncRead>),
}

pub struct LocalImageSourceResolver {
    root: Box<Path>,
}

impl LocalImageSourceResolver {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self { root: path.as_ref().into() }
    }
}

impl ImageSourceResolver for LocalImageSourceResolver {
    fn get(&self, identifier: &str) -> Result<ImageSource, std::io::Error> {
        todo!()
    }
}

pub trait ImageSourceResolver {
    fn get(&self, identifier: &str) -> Result<ImageSource, std::io::Error>;
}
