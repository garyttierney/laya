use std::path::Path;

use tokio::io::{AsyncRead, AsyncSeek};

use crate::storage::FileOrStream;

pub trait AsyncReadSeekable: AsyncRead + AsyncSeek {}
impl<R: AsyncRead + AsyncSeek> AsyncReadSeekable for R {}

pub trait ImageSourceResolver {
    fn get(&self, identifier: &str) -> Result<FileOrStream, std::io::Error>;
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
    fn get(&self, identifier: &str) -> Result<FileOrStream, std::io::Error> {
        todo!()
    }
}
