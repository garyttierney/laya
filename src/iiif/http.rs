use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;

use futures::{Stream, StreamExt};
use http_body::Frame;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty, Full, StreamBody};
use hyper::body::{Bytes, Incoming};
use hyper::header::{CONTENT_TYPE, HeaderValue, LAST_MODIFIED};
use hyper::{Request, Response, StatusCode};
use serde_json::{Value, json, to_string_pretty};
use tower::Service;
use tracing::{Instrument, error};

use super::service::{
    ImageServiceError, ImageServiceRequestKind, ImageServiceResponse, ImageServiceResponseKind,
};
use crate::iiif::ImageServiceRequest;
use crate::iiif::parse::ParseError as ImageRequestParseError;
use crate::storage::StorageError;

#[derive(Clone)]
pub struct HttpImageService<S>
where
    S: Clone,
{
    inner: S,
    prefix: String,
}

impl<S: Clone> HttpImageService<S> {
    pub(crate) fn new_with_prefix(image_service: S, prefix: &str) -> Self {
        Self { inner: image_service, prefix: prefix.to_string() }
    }
}

impl<S> tower::Service<Request<Incoming>> for HttpImageService<S>
where
    S: Service<ImageServiceRequest, Response = ImageServiceResponse, Error = ImageServiceError>
        + Send
        + Sync
        + Clone
        + 'static,
    S::Future: Send + Unpin,
{
    type Response = Response<BoxBody<Bytes, std::io::Error>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Send + Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, req: Request<Incoming>) -> Self::Future {
        Box::pin(Self::decode_request(req, self.prefix.clone(), self.inner.clone()))
    }

    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct BytesStream(Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send + Sync + Unpin>);

impl Stream for BytesStream {
    type Item = Result<Bytes, std::io::Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.as_mut().0.poll_next_unpin(cx)
    }
}

type HttpImageServiceBody = BoxBody<Bytes, std::io::Error>;
type HttpImageServiceResponse = Response<HttpImageServiceBody>;

const IMAGE_REQUEST_ROUTE: &str =
    "/<prefix>/<identifier>/<region>/<size>/<rotation>/<quality>.<format>";
const INFO_REQUEST_ROUTE: &str = "/<prefix>/<identifier>/info.json";

impl<S> HttpImageService<S>
where
    S: Service<ImageServiceRequest, Response = ImageServiceResponse, Error = ImageServiceError>
        + Send
        + Sync
        + Clone
        + 'static,
    S::Future: Send,
{
    pub async fn decode_request(
        req: Request<Incoming>,
        prefix: String,
        mut inner: S,
    ) -> Result<HttpImageServiceResponse, hyper::http::Error> {
        let request_path = req
            .uri()
            .path()
            .trim_start_matches(prefix.trim_end_matches("/"))
            .to_string();

        let request_span = tracing::Span::current();
        let request_method = req.method().to_string();
        let request = match request_path.as_str() {
            "/" => return ok_response("OK!"),
            _ => req.try_into(),
        };

        match request {
            Ok(request) => {
                let route = match &request {
                    ImageServiceRequest { kind: ImageServiceRequestKind::Info, .. } => {
                        INFO_REQUEST_ROUTE
                    }
                    ImageServiceRequest { kind: ImageServiceRequestKind::Image(..), .. } => {
                        IMAGE_REQUEST_ROUTE
                    }
                };

                request_span.record("otel.name", format!("{} {route}", request_method));

                match inner.call(request).instrument(request_span).await {
                    Ok(response) => response.try_into(),
                    Err(ImageServiceError::Storage(StorageError::NotFound)) => Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(text_body("Image file not found")),
                    Err(e) => {
                        error!("failed to handle an image service request: {e:?}");

                        Response::builder()
                            .status(StatusCode::INTERNAL_SERVER_ERROR)
                            .body(text_body("An internal error occurred"))
                    }
                }
            }
            Err(e) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(text_body(e.to_string())),
        }
    }
}

impl TryInto<HttpImageServiceResponse> for ImageServiceResponse {
    type Error = hyper::http::Error;

    fn try_into(self) -> Result<HttpImageServiceResponse, Self::Error> {
        let mut response = Response::builder();
        let headers = response.headers_mut().unwrap();

        if let Some(Ok(value)) = self
            .last_modified_time
            .map(httpdate::fmt_http_date)
            .map(|value| HeaderValue::from_str(&value))
        {
            headers.append(LAST_MODIFIED, value);
        }

        match self.kind {
            ImageServiceResponseKind::CacheHit => Response::builder()
                .status(StatusCode::NOT_MODIFIED)
                .body(BodyExt::boxed(Empty::new().map_err(|_| unreachable!()))),

            ImageServiceResponseKind::Image(image) => {
                let body = StreamBody::new(image.data.map(|data| data.map(Frame::data)));

                response
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, image.media_type.canonicalize().to_string())
                    .body(BodyExt::boxed(body))
            }
            ImageServiceResponseKind::Info(info) => {
                let mut document = json!({
                    "@context": "http://iiif.io/api/image/3/context.json",
                    "type": "ImageService3",
                    "protocol": "http://iiif.io/api/image",
                    "profile": "level0",
                    "width": info.width,
                    "height": info.height,
                });

                if let Some(sizes) = &info.sizes {
                    let sizes_documents: Vec<Value> = sizes
                        .iter()
                        .map(|size| {
                            json!({
                                "type": "Size",
                                "width": size.width,
                                "height": size.height,
                            })
                        })
                        .collect();

                    document["sizes"] = json!(sizes_documents)
                }

                if let Some(tiles) = &info.tiles {
                    let tile_documents: Vec<Value> = tiles
                        .iter()
                        .map(|tile| {
                            json!({
                                "type": "Tile",
                                "width": tile.width,
                                "height": tile.height,
                                "scaleFactors": tile.scale_factors
                            })
                        })
                        .collect();

                    document["tiles"] = json!(tile_documents);
                }

                if let Some(rights) = &info.rights {
                    document["rights"] = json!(rights);
                }

                let body = to_string_pretty(&document).expect("failed to serialize info.json");

                response
                    .status(StatusCode::OK)
                    .header(CONTENT_TYPE, "application/ld+json")
                    .body(text_body(body))
            }
        }
    }
}

pub fn text_body<S: Into<String>>(body: S) -> HttpImageServiceBody {
    Full::<Bytes>::from(body.into())
        .map_err(|_| unreachable!())
        .boxed()
}

fn ok_response<S: Into<String>>(body: S) -> Result<HttpImageServiceResponse, hyper::http::Error> {
    Response::builder()
        .status(StatusCode::OK)
        .body(text_body(body))
}

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
