use std::error::Error;
use std::fmt::Display;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;
use std::time::SystemTime;

use futures::FutureExt;
use hyper::Request;
use hyper::body::Incoming;
use hyper::header::IF_MODIFIED_SINCE;
use tower::Service;

use super::http::IiifRequestError;
use super::{Format, Quality, Region, Rotation, Size};
use crate::image::info::ImageInfo;
use crate::image::{BoxedImage, Image, ImageReader, ImageStream};
use crate::storage::{StorageError, StorageProvider};

pub enum ImageServiceResponseKind {
    Info(ImageInfo),
    Image(ImageStream),
    CacheHit,
}

pub struct ImageServiceResponse {
    pub kind: ImageServiceResponseKind,
    pub last_modified_time: Option<SystemTime>,
}

#[derive(Debug, PartialEq)]
pub struct ImageServiceRequest {
    pub(crate) identifier: String,
    pub(crate) kind: ImageServiceRequestKind,
    pub(crate) last_access_time: Option<SystemTime>,
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

impl TryFrom<Request<Incoming>> for ImageServiceRequest {
    type Error = IiifRequestError;

    fn try_from(req: Request<Incoming>) -> Result<Self, Self::Error> {
        let last_access_time = req
            .headers()
            .get(IF_MODIFIED_SINCE)
            .and_then(|value| httpdate::parse_http_date(value.to_str().ok()?).ok());

        Ok(req
            .uri()
            .path()
            .parse::<ImageServiceRequest>()?
            .with_last_access_time(last_access_time))
    }
}

impl ImageServiceRequest {
    pub fn info<S: Into<String>>(identifier: S) -> Self {
        ImageServiceRequest {
            identifier: identifier.into(),
            kind: ImageServiceRequestKind::Info,
            last_access_time: None,
        }
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
            last_access_time: None,
            kind: ImageServiceRequestKind::Image(ImageParameters {
                region,
                size,
                rotation,
                quality,
                format,
            }),
        }
    }

    pub fn with_last_access_time(self, last_access_time: Option<SystemTime>) -> Self {
        Self { last_access_time, ..self }
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
    pub fn new<S, R>(storage: S, reader: R) -> ImageService
    where
        S: StorageProvider + 'static,
        R: ImageReader + 'static,
    {
        Self { storage: Arc::new(storage), reader: Arc::from(reader) }
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

            if req
                .last_access_time
                .zip(data.last_modified)
                .is_some_and(|(atime, mtime)| atime >= mtime)
            {
                return Ok(ImageServiceResponse {
                    kind: ImageServiceResponseKind::CacheHit,
                    last_modified_time: data.last_modified,
                });
            }

            let image = reader.read(data.name, data.content).await;
            let kind = match req.kind {
                ImageServiceRequestKind::Info => handle_info_request(image)
                    .await
                    .map(ImageServiceResponseKind::Info),
                ImageServiceRequestKind::Image(params) => handle_image_request(image, params)
                    .await
                    .map(ImageServiceResponseKind::Image),
            }?;

            Ok(ImageServiceResponse { kind, last_modified_time: data.last_modified })
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
    image: BoxedImage,
    params: ImageParameters,
) -> Result<ImageStream, ImageServiceError> {
    // let (decoded_tx, decoded_rx) = mpsc::channel(16);
    // let decode_task = tokio::task::spawn_blocking(move || {
    //     let decoder = image.open_region(params.region);
    //     let mut buffer = BytesMut::default();

    //     while decoder.process(&mut buffer) {
    //         let buf = std::mem::replace(&mut buffer, BytesMut::default());

    //         if let Err(_) = decoded_tx.blocking_send(buf.freeze()) {
    //             warn!("image decoding task was cancelled prematurely");
    //             return;
    //         }
    //     }
    // });

    // let (encoded_tx, encoded_rx) = mpsc::channel(16);
    // let encode_task = tokio::task::spawn_blocking(move || {
    //     let encoder = todo!() /* API TBD */;
    //     let mut buffer = BytesMut::default();

    //     while let Some(data) = decoded_rx.blocking_recv() {
    //         encoder.encode(data, &mut buffer);
    //     }
    // });
    todo!()
}
