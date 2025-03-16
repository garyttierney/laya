use std::future::Future;
use std::pin::Pin;

use chrono::Timelike;
use opendal::{Builder, Operator};

use super::{FileOrStream, StorageError, StorageObject, StorageProvider};

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

impl From<opendal::Error> for StorageError {
    fn from(value: opendal::Error) -> Self {
        match value.kind() {
            opendal::ErrorKind::NotFound => StorageError::NotFound,
            _ => StorageError::Other(value.to_string()),
        }
    }
}

impl StorageProvider for OpenDalStorageProvider {
    fn open(
        &self,
        id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<StorageObject, StorageError>> + Send + 'static>> {
        let operator = self.operator.clone();
        let path = id.to_string();

        Box::pin(open(operator, path))
    }
}

async fn open(operator: Operator, path: String) -> Result<StorageObject, StorageError> {
    let stat = operator.stat(&path).await?;
    let reader = operator
        .reader(&path)
        .await?
        .into_futures_async_read(..)
        .await?;

    Ok(StorageObject {
        name: Some(path),
        content: FileOrStream::Stream(Box::new(reader)),
        last_modified: stat
            .last_modified()
            .map(|utc| utc.with_nanosecond(0).unwrap().into()),
    })
}
