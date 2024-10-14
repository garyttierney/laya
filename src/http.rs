use std::convert::Infallible;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Empty, Full};
use hyper::body::{Body, Bytes, Incoming};
use hyper::service::Service;
use hyper::{Method, Request, Response, StatusCode};

use crate::iiif::parse::ParseError as ImageRequestParseError;
use crate::iiif::{Format, ImageRequest, Quality, Region, Rotation, Size};

const PREFIX: &str = "/"; // TODO: read this from config

#[tracing::instrument]
pub async fn handle_request(
    req: Request<Incoming>,
    images: (),
) -> Result<Response<BoxBody<Bytes, std::io::Error>>, hyper::http::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, p) if p.ends_with("info.json") => info_request(p),
        (&Method::GET, p) if p.starts_with(PREFIX) => image_request(p, images).await,
        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Empty::new().map_err(|e| unreachable!()).boxed()),
    }
}

#[tracing::instrument]
async fn image_request(
    path: &str,
    source: (),
) -> Result<Response<BoxBody<Bytes, std::io::Error>>, hyper::http::Error> {
    let request = match decode_image_request(path) {
        Ok(r) => r,
        Err(e) => return bad_request(e.to_string()),
    };

    // let Ok(image) = todo!("fix this"); source.resolve(request.identifier()).await else {
    //     return Ok(bad_request("io error")); // TODO
    // };

    Response::builder()
        .status(StatusCode::OK)
        .body(Empty::new().map_err(|e| unreachable!()).boxed())
}

fn decode_image_request(path: &str) -> Result<ImageRequest, ImageRequestError> {
    let mut segments = path.split('/');
    debug_assert!(segments.next().is_some_and(|s| s.is_empty()));

    let identifier = segments
        .next()
        .ok_or(ImageRequestError::UriMissingElement("identifier"))
        .and_then(|input| {
            urlencoding::decode(input).map_err(|err| ImageRequestError::UriNotUtf8("identifier"))
        })?;

    let region = segments
        .next()
        .ok_or(ImageRequestError::UriMissingElement("region"))?
        .parse::<Region>()
        .map_err(ImageRequestError::from)?;

    let size = segments
        .next()
        .ok_or(ImageRequestError::UriMissingElement("size"))?
        .parse::<Size>()
        .map_err(ImageRequestError::from)?;

    let rotation = segments
        .next()
        .ok_or(ImageRequestError::UriMissingElement("rotation"))?
        .parse::<Rotation>()
        .map_err(ImageRequestError::from)?;

    let (quality, format) = segments
        .next()
        .ok_or(ImageRequestError::UriMissingElement("quality"))?
        .split_once('.')
        .ok_or(ImageRequestError::UriMissingElement("format"))?;

    let quality = quality
        .parse::<Quality>()
        .map_err(ImageRequestError::ParseError)?;

    let format = format
        .parse::<Format>()
        .map_err(ImageRequestError::ParseError)?;

    Ok(ImageRequest::new(identifier, region, size, rotation, quality, format))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImageRequestError {
    /// If the URI did not contain an expected element.
    UriMissingElement(&'static str),

    /// If the URI contained a text element that was not in UTF-8 (which is an RFC6570 violation).
    UriNotUtf8(&'static str),

    /// If the request contained input that could not be parsed.
    ParseError(ImageRequestParseError),
}

impl Error for ImageRequestError {}

impl Display for ImageRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImageRequestError::UriMissingElement(element) => {
                write!(f, "Request path missing {element}.")
            }
            ImageRequestError::ParseError(error) => Display::fmt(error, f),
            ImageRequestError::UriNotUtf8(element) => {
                write!(f, "Request path {element} was not in UTF-8.")
            }
        }
    }
}

impl From<ImageRequestParseError> for ImageRequestError {
    fn from(value: ImageRequestParseError) -> Self {
        ImageRequestError::ParseError(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InfoRequestError {}

fn info_request(
    path: &str,
) -> Result<Response<BoxBody<Bytes, std::io::Error>>, hyper::http::Error> {
    unimplemented!()
}

fn bad_request<I: Into<Bytes>>(
    body: I,
) -> Result<Response<BoxBody<Bytes, std::io::Error>>, hyper::http::Error> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Full::new(body.into()).map_err(|e| match e {}).boxed())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::iiif::Scale;

    #[test]
    fn decode_basic_image_request() {
        let request = decode_image_request("/abcd1234/full/max/0/default.jpg");
        assert_eq!(
            request,
            Ok(ImageRequest::new(
                "abcd1234",
                Region::Full,
                Size::new(Scale::Max),
                Rotation::new(0.0),
                Quality::Default,
                Format::Jpg,
            ))
        );
    }

    #[test]
    fn decode_encoded_image_request() {
        // Image API 3.0, s 9: to-encode = "/" / "?" / "#" / "[" / "]" / "@" / "%"
        let request = decode_image_request("/a%2F%3F%23%5B%5D%40%25z/full/max/0/default.jpg");
        assert_eq!(
            request,
            Ok(ImageRequest::new(
                "a/?#[]@%z",
                Region::Full,
                Size::new(Scale::Max),
                Rotation::new(0.0),
                Quality::Default,
                Format::Jpg,
            ))
        );
    }
}
