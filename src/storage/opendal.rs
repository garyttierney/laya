use std::future::Future;
use std::pin::Pin;

use opendal::{Builder, Operator};

use super::{FileOrStream, StorageError, StorageProvider};

pub struct OpenDalStorageProvider {
    operator: Operator,
}

impl OpenDalStorageProvider {
    pub fn new<T: Builder>(builder: T) -> Result<Self, opendal::Error> {
        let operator = Operator::new(builder)?
            .layer(opendal::layers::TracingLayer)
            .finish();

        Ok(Self { operator })
    }
}

impl StorageProvider for OpenDalStorageProvider {
    fn open(
        &self,
        id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<FileOrStream, StorageError>> + Send + 'static>> {
        let operator = self.operator.clone();
        let path = id.to_string();

        Box::pin(open(operator, path))
    }
}

async fn open(operator: Operator, path: String) -> Result<FileOrStream, StorageError> {
    let reader = operator
        .reader(&path)
        .await
        .unwrap()
        .into_futures_async_read(..)
        .await
        .expect("couldn't create an async reader");

    Ok(FileOrStream::Stream(Box::new(reader)))
}
