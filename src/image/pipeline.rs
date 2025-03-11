use super::ImageReader;
use crate::storage::StorageProvider;

#[derive(Default)]
pub struct ImagePipelineBuilder<S: StorageProvider, R: ImageReader> {
    storage: Option<S>,
    reader: Option<R>,
}

impl<L: StorageProvider, R: ImageReader> ImagePipelineBuilder<L, R> {
    pub fn new() -> Self {
        Self { storage: None, reader: None }
    }

    pub fn with_storage(mut self, storage: L) -> Self {
        self.storage = Some(storage);
        self
    }

    pub fn with_reader(mut self, reader: R) -> Self {
        self.reader = Some(reader);
        self
    }

    pub fn build(self) -> ImagePipeline<L, R> {
        ImagePipeline {
            storage: self.storage.expect("no locator set"),
            reader: self.reader.expect("no reader set"),
        }
    }
}

#[allow(unused)]
pub struct ImagePipeline<L: StorageProvider, R: ImageReader> {
    pub(crate) storage: L,
    pub(crate) reader: R,
}
