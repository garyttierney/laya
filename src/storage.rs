use std::error::Error;
use std::fmt::Display;
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::time::SystemTime;

use futures::AsyncRead;

pub mod opendal;

pub type FileStreamProvider = Box<dyn FnOnce(&Path) -> Box<dyn AsyncRead> + Send>;

/// An object stored by a storage provider.
///
/// This structure represents a stored object with optional metadata
/// such as a name and last modification time, along with its contents.
pub struct StorageObject {
    pub name: Option<String>,
    pub last_modified: Option<SystemTime>,
    pub content: FileOrStream,
}

/// Provides storage for data identified by unique string identifiers.
pub trait StorageProvider: Send + Sync {
    /// Opens a handle to the random-access storage identified by the unique id `id`.
    /// The storage may point to an asynchronous stream, or a locally available file.
    fn open(
        &self,
        id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<StorageObject, StorageError>> + Send>>;

    fn healthcheck(&self) -> Pin<Box<dyn Future<Output = Result<(), StorageError>> + Send>> {
        Box::pin(futures::future::ready(Ok(())))
    }
}

impl<T: StorageProvider> StorageProvider for Box<T> {
    fn open(
        &self,
        id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<StorageObject, StorageError>> + Send>> {
        T::open(self, id)
    }
}

impl StorageProvider for Box<dyn StorageProvider> {
    fn open(
        &self,
        id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<StorageObject, StorageError>> + Send>> {
        <dyn StorageProvider>::open(self, id)
    }
}

/// A path to a file encapsulated with a factory method that can provide an asynchronous stream if
/// necessary.
pub struct FileStream {
    pub path: Box<Path>,
    pub stream_factory: FileStreamProvider,
}

/// A source of data, coming from a local file or asynchronous stream.
///
/// This enum represents the different data can be loaded.
///
/// The reason `File` and `Stream` are separate despite `AsyncRead` being
/// able to represent a file is so decoders can decide to make optimizations
/// based on direct filesystem access, such as using memory mapped files or
/// DMA.
///
/// If the data source does not optimize for locally available files, a `Box<dyn AsyncRead>` can be
/// obtained from [FileOrStream::as_stream()]
pub enum FileOrStream {
    /// Storage represented by a file on the filesystem.
    File(FileStream),

    /// Storage represented by a stream.
    Stream(Box<dyn AsyncRead + Send>),
}

impl FileOrStream {
    /// Get the contents of this value as an asynchronus stream, regardless of local availability.
    pub fn as_stream(self) -> Box<dyn AsyncRead> {
        match self {
            FileOrStream::File(stream) => (stream.stream_factory)(&stream.path),
            FileOrStream::Stream(stream) => stream,
        }
    }
}

#[derive(Debug)]
pub enum StorageError {
    AccessDenied,
    NotFound,
    Other(String),
}

impl Error for StorageError {}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::AccessDenied => write!(f, "access was denied"),
            StorageError::NotFound => write!(f, "data could not be found"),
            StorageError::Other(reason) => write!(f, "other: {reason}"),
        }
    }
}
