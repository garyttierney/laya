use std::error::Error;
use std::fmt::Display;
use std::future::Future;
use std::pin::{self, Pin};
use std::sync::{Arc, Mutex};
use std::task::Poll;

use futures::Stream;
use hyper::body::Incoming;
use hyper::Request;
use tower::Service;

use super::http::IiifRequestError;
use super::info::ImageInfo;
use super::{Format, Quality, Region, Rotation, Size};
use crate::image::{BoxedImage, Image, ImagePipeline, ImageReader};
use crate::storage::StorageProvider;

pub enum ImageServiceResponse {
    Info(ImageInfo),
    Image(Box<dyn Stream<Item = bytes::Bytes> + Unpin + Send + Sync>),
}

#[derive(Debug, PartialEq)]
pub enum ImageServiceRequest {
    Info {
        identifier: String,
    },
    Image {
        identifier: String,
        region: Region,
        size: Size,
        rotation: Rotation,
        quality: Quality,
        format: Format,
    },
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
        ImageServiceRequest::Info { identifier: identifier.into() }
    }

    pub fn image<S: Into<String>>(
        identifier: S,
        region: Region,
        size: Size,
        rotation: Rotation,
        quality: Quality,
        format: Format,
    ) -> ImageServiceRequest {
        ImageServiceRequest::Image {
            identifier: identifier.into(),
            region,
            size,
            rotation,
            quality,
            format,
        }
    }
}

#[derive(Debug)]
pub enum ImageServiceError {}

impl Error for ImageServiceError {}
impl Display for ImageServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Clone)]
pub struct ImageService {
    storage: Arc<tokio::sync::Mutex<dyn StorageProvider + Send + Sync>>,
    reader: Arc<dyn ImageReader + Send + Sync>,
}

impl ImageService {
    pub fn new<S, R>(pipeline: ImagePipeline<S, R>) -> ImageService
    where
        S: StorageProvider + Send + Sync + 'static,
        R: ImageReader + Send + Sync + 'static,
    {
        let storage = Arc::new(tokio::sync::Mutex::new(pipeline.storage));
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
            match req {
                ImageServiceRequest::Info { identifier } => {
                    let data = storage.lock().await.open(&identifier).await.unwrap();
                    let mut image = reader.read(data).await;
                    let info = image.info();

                    Ok(ImageServiceResponse::Info(info))
                }
                ImageServiceRequest::Image {
                    identifier,
                    region,
                    size,
                    rotation,
                    quality,
                    format,
                } => todo!(),
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
