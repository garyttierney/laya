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
use tokio::sync::mpsc::error::SendError;
use tower::Service;
use tracing::{info, warn};

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
    pipeline: Arc<ImagePipeline<Box<dyn StorageProvider>, Box<dyn ImageReader>>>, 
}

impl ImageService {
    pub fn new<S, R>(pipeline: ImagePipeline<S, R>) -> ImageService
    where
        S: StorageProvider,
        R: ImageReader + 'static,
    {
        let boed = pipeline.boxed();

        todo!()
        // Self { pipeline: Arc::new(pipeline.boxed()) }
    }
}

impl Service<ImageServiceRequest> for ImageService {
    type Response = ImageServiceResponse;
    type Error = ImageServiceError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&mut self, req: ImageServiceRequest) -> Self::Future {
        todo!()
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
    let (decoded_tx, decoded_rx) = mpsc::channel(16);
    let decode_task = tokio::task::spawn_blocking(move || {
        let decoder = image.open_region(params.region);
        let mut buffer = BytesMut::default();

        while decoder.process(&mut buffer) {
            let buf = std::mem::replace(&mut buffer, BytesMut::default());

            if let Err(_) = decoded_tx.blocking_send(buf.freeze()) {
                warn!("image decoding task was cancelled prematurely");
                return;
            }
        }
    });

    let (encoded_tx, encoded_rx) = mpsc::channel(16);
    let encode_task = tokio::task::spawn_blocking(move || {
        let encoder = todo!() /* API TBD */;
        let mut buffer = BytesMut::default();

        while let Some(data) = decoded_rx.blocking_recv() {
            encoder.encode(data, &mut buffer);
        }
    });
    todo!()
}
