use std::convert::Infallible;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::future::{ready, Future};
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;

use futures::Stream;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::{Body, Bytes, Incoming};
use hyper::service::Service;
use hyper::{Method, Request, Response, StatusCode};
use tracing::{error, info};

use crate::iiif::info::ImageInfo;
use crate::iiif::parse::ParseError as ImageRequestParseError;
use crate::iiif::{Format, IiifRequest, Quality, Region, Rotation, Size};
use crate::image::{ImageMetadataResolver, ImagePipeline, ImageSourceResolver};

const PREFIX: &str = "/"; // TODO: read this from config

pub struct IiifImageService<L: ImageSourceResolver, R: ImageMetadataResolver> {
    options: Arc<IiifImageServiceOptions>,
    pipeline: Arc<ImagePipeline<L, R>>,
}

impl<L: ImageSourceResolver, R: ImageMetadataResolver> Clone for IiifImageService<L, R> {
    fn clone(&self) -> Self {
        Self { options: self.options.clone(), pipeline: self.pipeline.clone() }
    }
}

impl<L: ImageSourceResolver, R: ImageMetadataResolver> IiifImageService<L, R> {
    pub fn new_with_prefix<S: Into<String>>(pipeline: ImagePipeline<L, R>, prefix: S) -> Self {
        Self {
            pipeline: Arc::new(pipeline),
            options: Arc::new(IiifImageServiceOptions { prefix: prefix.into() }),
        }
    }
}

#[derive(Clone)]
pub struct IiifImageServiceOptions {
    prefix: String,
}

impl<L: ImageSourceResolver + Send + Sync, R: ImageMetadataResolver + Send + Sync>
    tower::Service<Request<Incoming>> for IiifImageService<L, R>
{
    type Response = Response<BoxBody<Bytes, std::io::Error>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Sync + Send + Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, req: Request<Incoming>) -> Self::Future {
        let options = self.options.clone();
        let pipeline = self.pipeline.clone();

        Box::pin(dispatch_request(req, options))
    }

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub enum IiifImageServiceResponse {
    Info(ImageInfo),
    Image(Box<dyn Stream<Item = Bytes>>),
}

pub async fn dispatch_request(
    req: Request<Incoming>,
    options: Arc<IiifImageServiceOptions>,
) -> Result<Response<BoxBody<Bytes, std::io::Error>>, hyper::http::Error> {
    let request = decode_request(req, &options);

    match request {
        Ok(IiifRequest::Image { .. }) => todo!(),
        Ok(IiifRequest::Info { identifier }) => info_request(&identifier),
        Err(e) => bad_request(e.to_string()),
    }
}

#[tracing::instrument(skip_all, ret, err)]
fn decode_request(
    req: Request<Incoming>,
    options: &IiifImageServiceOptions,
) -> Result<IiifRequest, IiifRequestError> {
    req.uri()
        .path()
        .trim_start_matches(&options.prefix.trim_end_matches("/"))
        .parse::<IiifRequest>()
}

// #[tracing::instrument(skip_all, err)]
// async fn image_request(
//     request: ImageRequest,
//     source: (),
// ) -> Result<Response<BoxBody<Bytes, std::io::Error>>, hyper::http::Error> {
//     // let Ok(image) = todo!("fix this"); source.resolve(request.identifier()).await else {
//     //     return Ok(bad_request("io error")); // TODO
//     // };

//     Response::builder()
//         .status(StatusCode::OK)
//         .body(Empty::new().map_err(|e| unreachable!()).boxed())
// }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IiifRequestError {
    /// If the URI did not contain an expected element.
    UriMissingElement(&'static str),

    /// If the URI contained a text element that was not in UTF-8 (which is an RFC6570 violation).
    UriNotUtf8(&'static str),

    /// If the request contained input that could not be parsed.
    ParseError(ImageRequestParseError),
}

impl Error for IiifRequestError {}

impl Display for IiifRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IiifRequestError::UriMissingElement(element) => {
                write!(f, "Request path missing {element}.")
            }
            IiifRequestError::ParseError(error) => Display::fmt(error, f),
            IiifRequestError::UriNotUtf8(element) => {
                write!(f, "Request path {element} was not in UTF-8.")
            }
        }
    }
}

impl From<ImageRequestParseError> for IiifRequestError {
    fn from(value: ImageRequestParseError) -> Self {
        IiifRequestError::ParseError(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InfoRequestError {}

#[tracing::instrument(ret, err)]
fn info_request(id: &str) -> Result<Response<BoxBody<Bytes, std::io::Error>>, hyper::http::Error> {
    unimplemented!()
}

fn bad_request<I: Into<Bytes>>(
    body: I,
) -> Result<Response<BoxBody<Bytes, std::io::Error>>, hyper::http::Error> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Full::new(body.into()).map_err(|e| match e {}).boxed())
}
