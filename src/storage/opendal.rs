use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Timelike;
use hyper::Uri;
use opendal::layers::TracingLayer;
use opendal::services::{Fs, S3};
use opendal::{Builder, Operator};
use tracing::info;

use super::{FileOrStream, StorageError, StorageObject, StorageProvider};

pub struct OpenDalStorageProvider {
    path: PathBuf,
}

impl OpenDalStorageProvider {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
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
        Box::pin(open(self.path.clone(), id.to_string()))
    }
}

#[tracing::instrument]
async fn open(local_root: PathBuf, path: String) -> Result<StorageObject, StorageError> {
    let (operator, path) = match path.parse::<Uri>() {
        Ok(uri) if uri.scheme_str() == Some("s3") => {
            let (region, bucket_and_path) = uri
                .path()
                .split_once('/')
                .ok_or(StorageError::Other("invalid S3 URI specification".into()))?;
            let (bucket, bucket_key) = bucket_and_path
                .split_once('/')
                .ok_or(StorageError::Other("invalid S3 bucket/key specification".into()))?;

            info!(
                region = region,
                bucket = bucket,
                bucket_key = bucket_key,
                "recognised image path as S3 object"
            );

            (
                Operator::new(S3::default().region(region).bucket(bucket))?
                    .layer(TracingLayer)
                    .finish(),
                bucket_key.to_string(),
            )
        }
        _ => (
            Operator::new(
                Fs::default().root(local_root.to_str().expect("invalid root path provided")),
            )?
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
