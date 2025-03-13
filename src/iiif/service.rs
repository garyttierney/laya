use std::error::Error;
use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;

use bytes::BytesMut;
use futures::{SinkExt, Stream};
use hyper::body::Incoming;
use hyper::Request;
use kaduceus::KakaduContext;
use tokio::sync::mpsc;
use tower::Service;
use tracing::info;

use super::http::IiifRequestError;
use super::info::ImageInfo;
use super::{Format, Quality, Region, Rotation, Size};
use crate::image::{BoxedImage, Image, ImagePipeline, ImageReader, ImageStream};
use crate::storage::{FileOrStream, StorageError, StorageProvider};

pub enum ImageServiceResponse {
    Info(ImageInfo),
    Image(ImageStream),
}

#[derive(Debug, PartialEq)]
pub struct ImageServiceRequest {
    pub(crate) identifier: String,
    pub(crate) kind: ImageServiceRequestKind,
}

#[derive(Debug, PartialEq)]
pub enum ImageServiceRequestKind {
    Info,
    Image(ImageParameters),
}

#[derive(Debug, PartialEq)]
pub struct ImageParameters {
    region: Region,
    size: Size,
    rotation: Rotation,
    quality: Quality,
    format: Format,
}

impl TryFrom<&Request<Incoming>> for ImageServiceRequest {
    type Error = IiifRequestError;

    fn try_from(req: &Request<Incoming>) -> Result<Self, Self::Error> {
        let path = req.uri().path();
        path.parse()
    }
}

impl ImageServiceRequest {
    pub fn info<S: Into<String>>(identifier: S) -> Self {
        ImageServiceRequest { identifier: identifier.into(), kind: ImageServiceRequestKind::Info }
    }

    pub fn image<S: Into<String>>(
        identifier: S,
        region: Region,
        size: Size,
        rotation: Rotation,
        quality: Quality,
        format: Format,
    ) -> Self {
        ImageServiceRequest {
            identifier: identifier.into(),
            kind: ImageServiceRequestKind::Image(ImageParameters {
                region,
                size,
                rotation,
                quality,
                format,
            }),
        }
    }
}

#[derive(Debug)]
pub enum ImageServiceError {
    Storage(StorageError),
}

impl Error for ImageServiceError {}
impl Display for ImageServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Clone)]
pub struct ImageService {
    storage: Arc<dyn StorageProvider>,
    reader: Arc<dyn ImageReader>,
}

impl ImageService {
    pub fn new<S, R>(pipeline: ImagePipeline<S, R>) -> ImageService
    where
        S: StorageProvider + 'static,
        R: ImageReader + Send + Sync + 'static,
    {
        let storage = Arc::new(pipeline.storage);
        let reader = Arc::new(pipeline.reader);

        Self { storage, reader }
    }
}

impl Service<ImageServiceRequest> for ImageService {
    type Response = ImageServiceResponse;
    type Error = ImageServiceError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, req: ImageServiceRequest) -> Self::Future {
        let storage = self.storage.clone();
        let reader = self.reader.clone();

        Box::pin(async move {
            let data = storage
                .open(&req.identifier)
                .await
                .map_err(ImageServiceError::Storage)?;
            let image = reader.read(data).await;

            match req.kind {
                ImageServiceRequestKind::Info => handle_info_request(image)
                    .await
                    .map(ImageServiceResponse::Info),
                ImageServiceRequestKind::Image(params) => handle_image_request(image, params)
                    .await
                    .map(ImageServiceResponse::Image),
            }
        })
    }

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

#[tracing::instrument(err, skip(image))]
async fn handle_info_request(mut image: BoxedImage) -> Result<ImageInfo, ImageServiceError> {
    let info = image.info();

    Ok(info)
}

#[tracing::instrument(err, skip(image))]
async fn handle_image_request(
    mut image: BoxedImage,
    params: ImageParameters,
) -> Result<ImageStream, ImageServiceError> {
    let (tx, rx) = mpsc::channel(16);
    let decode_task = tokio::task::spawn_blocking(move || {
        let decoder = image.open_region(params.region);
        let buffer = BytesMut::default();

        tx.blocking_send(buffer.freeze())
            .expect("failed to transmit decoded image buffer to encoder")
    });
    todo!()
}
