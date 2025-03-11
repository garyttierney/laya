use opendal::Operator;

use super::{FileOrStream, StorageError, StorageProvider};

pub struct OpenDalStorageProvider {
    operator: Operator,
}

impl OpenDalStorageProvider {
    pub fn new(operator: Operator) -> OpenDalStorageProvider {
        Self { operator }
    }
}

impl StorageProvider for OpenDalStorageProvider {
    async fn open(&self, id: &str) -> Result<FileOrStream, StorageError> {
        let reader = self
            .operator
            .reader(id)
            .await
            .unwrap()
            .into_futures_async_read(..)
            .await
            .expect("couldn't create an async reader");

        Ok(FileOrStream::Stream(Box::new(reader)))
    }
}
