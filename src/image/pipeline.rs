use std::{future::Future, sync::Arc};

use super::ImageReader;
use crate::storage::{FileOrStream, StorageError, StorageProvider};

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

pub struct ImagePipeline<S: StorageProvider, R: ImageReader> {
    pub(crate) storage: S,
    pub(crate) reader: R,
}

impl<S: StorageProvider + Sized, R: ImageReader + Sized> ImagePipeline<S, R> {
    pub fn boxed(self) -> ImagePipeline<Box<dyn StorageProvider + 'static>, Box<dyn ImageReader + 'static>> {
        ImagePipeline {
            storage: Box::new(self.storage),
            reader: Box::new(self.reader)
        }   
    }
}
