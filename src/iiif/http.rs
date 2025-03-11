use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::task::Poll;

use futures::{Stream, StreamExt};
use http_body::Frame;
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full, StreamBody};
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response, StatusCode};
use serde_json::{json, to_string_pretty, Value};
use tower::Service;

use super::service::{ImageServiceError, ImageServiceResponse};
use crate::iiif::info::ImageInfo;
use crate::iiif::parse::ParseError as ImageRequestParseError;
use crate::iiif::ImageServiceRequest;

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
    S::Future: Send + Sync,
{
    type Response = Response<BoxBody<Bytes, std::io::Error>>;
    type Error = hyper::http::Error;
    type Future = Pin<Box<dyn Sync + Send + Future<Output = Result<Self::Response, Self::Error>>>>;

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

pub struct BytesStream(Box<dyn Stream<Item = Bytes> + Send + Sync + Unpin>);

impl Stream for BytesStream {
    type Item = Bytes;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.as_mut().0.poll_next_unpin(cx)
    }
}

type HttpImageServiceBody = BoxBody<Bytes, std::io::Error>;
type HttpImageServiceResponse = Response<HttpImageServiceBody>;

impl<S> HttpImageService<S>
where
    S: Service<ImageServiceRequest, Response = ImageServiceResponse, Error = ImageServiceError>
        + Send
        + Sync
        + Clone
        + 'static,
    S::Future: Send + Sync,
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

        match request_path.as_str() {
            "/" => ok_response("OK!"),
            _ => {
                match request_path.parse() {
                    Ok(request) => {
                        let route = match &request {
                        ImageServiceRequest::Info { .. } => "/<prefix>/<identifier>/info.json",
                        ImageServiceRequest::Image { .. } => "/<prefix>/<identifier>/<region>/<size>/<rotation>/<quality>.<format>",
                    };

                        request_span.record("otel.name", format!("{} {route}", req.method()));

                        match inner.call(request).await {
                            Ok(ImageServiceResponse::Image(image)) => {
                                let body = StreamBody::new(
                                    BytesStream(image).map(|data| Ok(Frame::data(data))),
                                );

                                Response::builder()
                                    .status(StatusCode::OK)
                                    .body(BodyExt::boxed(body))
                            }
                            Ok(ImageServiceResponse::Info(info)) => info_response(info),
                            Err(_) => todo!(),
                        }
                    }
                    Err(e) => Response::builder().status(StatusCode::BAD_REQUEST).body(
                        Full::new(e.to_string().into())
                            .map_err(|_| unreachable!())
                            .boxed(),
                    ),
                }
            }
        }
    }
}

fn info_response(info: ImageInfo) -> Result<HttpImageServiceResponse, hyper::http::Error> {
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

    let json = to_string_pretty(&document).expect("failed to serialize image info document");

    ok_response(json)
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
