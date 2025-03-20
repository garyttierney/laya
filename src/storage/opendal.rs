use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Timelike;
use hyper::Uri;
use opendal::layers::TracingLayer;
use opendal::services::{Fs, S3};
use opendal::{Builder, Operator};

use super::{FileOrStream, StorageError, StorageObject, StorageProvider};

pub struct OpenDalStorageProvider;

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
        Box::pin(open(id.to_string()))
    }
}

#[tracing::instrument]
async fn open(path: String) -> Result<StorageObject, StorageError> {
    let (operator, path) = match path.parse::<Uri>() {
        Ok(uri) if uri.scheme_str() == Some("s3") => {
            let (region, bucket_and_path) = uri
                .path()
                .split_once('/')
                .ok_or(StorageError::Other("invalid S3 URI specification".into()))?;
            let (bucket, bucket_key) = bucket_and_path
                .split_once('/')
                .ok_or(StorageError::Other("invalid S3 bucket/key specification".into()))?;

            (
                Operator::new(S3::default().bucket(bucket).region(region))?
                    .layer(TracingLayer)
                    .finish(),
                bucket_key.to_string(),
            )
        }
        _ => (
            Operator::new(Fs::default().root("test-data"))?
                .layer(TracingLayer)
                .finish(),
            path.to_string(),
        ),
    };

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
