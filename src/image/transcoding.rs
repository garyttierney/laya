use std::error::Error;
use std::fmt::Display;
use std::io::Write;
use std::pin::pin;
use std::task::Poll;

use bytes::{BufMut, Bytes, BytesMut};
use decode::decode_task;
use encode::encode_task;
use futures::stream::Fuse;
use futures::{Stream, StreamExt};
use mediatype::MediaTypeBuf;
use mediatype::names::{IMAGE, JPEG};
use tokio::sync::mpsc::{self, Sender};
use tokio::task::JoinSet;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::CancellationToken;
use tracing::{error, info_span};

use super::{BoxedImage, ImageStream};
use crate::iiif::service::ImageParameters;

pub mod decode;
pub mod encode;

pub struct TranscodingPipeline {
    pub image: BoxedImage,
    pub params: ImageParameters,
}

#[derive(Debug)]
pub enum TranscodingError {
    Generic(String),
    Io(std::io::Error),
    Unknown,
}

impl Error for TranscodingError {}

impl From<std::io::Error> for TranscodingError {
    fn from(value: std::io::Error) -> Self {
        TranscodingError::Io(value)
    }
}

impl Display for TranscodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TranscodingError::Generic(message) => write!(f, "{message}"),
            TranscodingError::Io(err) => write!(f, "io error: {err}"),
            TranscodingError::Unknown => write!(f, "unknown error"),
        }
    }
}

/// A type that implements [Write] by buffering writes
/// and sending them to a channel when the buffer is full or when flushed.
///
/// `SenderWriter` maintains an internal buffer and a sender channel. When data is written,
/// it's first added to the buffer. Once the buffer reaches a certain size (4096 bytes),
/// or when explicitly flushed, the buffered data is sent through the channel.
pub struct SenderWriter {
    buffer: BytesMut,
    sender: Sender<Bytes>,
}

impl SenderWriter {
    pub fn new(sender: Sender<Bytes>) -> SenderWriter {
        Self { buffer: BytesMut::with_capacity(4096), sender }
    }
}

impl Write for SenderWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buffer.put(buf);

        if self.buffer.len() >= 4096 {
            self.flush()?;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.sender
            .blocking_send(std::mem::take(&mut self.buffer).freeze())
            .map_err(std::io::Error::other)
    }
}

impl TranscodingPipeline {
    pub fn run(self) -> ImageStream {
        let Self { mut image, params } = self;

        let info = image.info();
        let token = CancellationToken::new();
        let mut task_set = JoinSet::new();

        let decoder_token = token.clone();
        let decoder_span = info_span!("image_decoder", decoder = "kakadu");
        let (decoded_tx, decoded_rx) = mpsc::channel(4);

        task_set.spawn_blocking(move || -> Result<(), TranscodingError> {
            decoder_span.in_scope(|| decode_task(decoder_token, image, params, decoded_tx))
        });

        let encoder_token = token.clone();
        let encoder_span = info_span!("image_encoder", encoder = "mozjpeg");
        let (encoded_tx, encoded_rx) = mpsc::channel(4);

        task_set.spawn_blocking(move || -> Result<(), TranscodingError> {
            encoder_span.in_scope(|| encode_task(encoder_token, decoded_rx, encoded_tx, info))
        });

        ImageStream {
            media_type: MediaTypeBuf::new(IMAGE, JPEG),
            data: Box::new(TranscodedStream {
                task_set,
                token,
                receiver: ReceiverStream::new(encoded_rx).fuse(),
            }),
        }
    }
}

pub struct TranscodedStream {
    task_set: JoinSet<Result<(), TranscodingError>>,
    token: CancellationToken,
    receiver: Fuse<ReceiverStream<Bytes>>,
}

impl Stream for TranscodedStream {
    type Item = Result<Bytes, std::io::Error>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let task_set = &mut this.task_set;

        while let Poll::Ready(Some(res)) = task_set.poll_join_next(cx) {
            match res {
                Ok(Err(e)) => {
                    error!("An error occurred during transcoding: {e}");
                    this.token.cancel();

                    return Poll::Ready(Some(Err(std::io::Error::other(e))));
                }
                Err(e) => {
                    error!("Transcoding task panicked: {e}");
                    this.token.cancel();

                    return Poll::Ready(Some(Err(std::io::Error::other(e))));
                }
                _ => {} // Task completed successfully
            }
        }

        let inner = pin!(&mut this.receiver);

        if let Poll::Ready(Some(msg)) = inner.poll_next(cx) {
            return Poll::Ready(Some(Ok(msg)));
        }

        if this.task_set.is_empty() {
            return Poll::Ready(None);
        }

        Poll::Pending
    }
}
