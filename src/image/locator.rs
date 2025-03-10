use std::path::Path;

use crate::storage::FileOrStream;

pub trait ImageSourceResolver {
    fn get(&self, identifier: &str) -> Result<FileOrStream, std::io::Error>;
}

#[allow(unused)]
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
