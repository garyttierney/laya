use bytes::Bytes;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_util::sync::CancellationToken;

use super::{SenderWriter, TranscodingError};
use crate::image::info::ImageInfo;

pub fn encode_task(
    cancellation_token: CancellationToken,
    mut input_channel: Receiver<Bytes>,
    output_channel: Sender<Bytes>,
    info: ImageInfo,
) -> Result<(), TranscodingError> {
    std::panic::catch_unwind(move || {
        let mut compressor = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_EXT_RGB);
        compressor.set_size(info.width as usize, info.height as usize);

        let writer = SenderWriter::new(output_channel);
        let mut output = compressor.start_compress(writer)?;

        loop {
            if cancellation_token.is_cancelled() {
                return Ok(());
            }

            if let Some(input) = input_channel.blocking_recv() {
                output.write_scanlines(&input[..])?;
            } else {
                break;
            }
        }

        output.finish()?;

        Ok(())
    })
    .map_err(|err| {
        if let Ok(mut err) = err.downcast::<String>() {
            TranscodingError::Generic(std::mem::take(&mut *err))
        } else {
            TranscodingError::Unknown
        }
    })?
}
