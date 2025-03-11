use std::error::Error;
use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;

use futures::Stream;
use hyper::body::Incoming;
use hyper::Request;
use tower::Service;

use super::http::IiifRequestError;
use super::info::ImageInfo;
use super::{Format, Quality, Region, Rotation, Size};

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
pub struct ImageService;

impl Service<ImageServiceRequest> for ImageService {
    type Response = ImageServiceResponse;
    type Error = ImageServiceError;
    type Future = Pin<Box<dyn Sync + Send + Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, req: ImageServiceRequest) -> Self::Future {
        Box::pin(Self::handle(req))
    }

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

impl ImageService {
    pub async fn handle(
        request: ImageServiceRequest,
    ) -> Result<ImageServiceResponse, ImageServiceError> {
        unimplemented!()
    }
}
