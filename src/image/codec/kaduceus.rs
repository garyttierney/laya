use std::future::Future;
use std::io::Write;
use std::pin::Pin;
use std::sync::Arc;

use bytes::{BufMut, BytesMut};
use kaduceus::{KakaduContext, KakaduDecompressor, KakaduImage};
use tokio::runtime::{Builder, Runtime};
use tracing::info;

use super::ImageReader;
use crate::iiif::{Dimension, Region};
use crate::image::info::{ImageInfo, PreferredSize, Tile};
use crate::image::{AbsoluteRegion, BoxedImage, Image, ImageDecoder};
use crate::storage::FileOrStream;

pub struct KaduceusImageReader {
    context: KakaduContext,
    executor: Arc<Runtime>,
}

impl KaduceusImageReader {
    pub fn new(context: KakaduContext) -> Self {
        Self {
            context,
            executor: Arc::new(Builder::new_multi_thread().enable_all().build().unwrap()),
        }
    }
}

impl Image for KakaduImage {
    fn info(&mut self) -> ImageInfo {
        let info = KakaduImage::info(self);
        let tiles = vec![Tile {
            width: info.tile_width,
            height: Some(info.tile_height),
            scale_factors: (0..info.dwt_levels).map(|level| 1 << level).collect(),
        }];

        let mut sizes = vec![];
        for level in 0..info.dwt_levels {
            let scaling_factor = 1 << (info.dwt_levels - level);

            sizes.push(PreferredSize {
                width: info.width / scaling_factor,
                height: info.height / scaling_factor,
            });
        }

        ImageInfo {
            width: info.width,
            height: info.height,
            max_width: Some(info.width),
            max_height: Some(info.height),
            max_area: None,
            sizes: Some(sizes),
            tiles: Some(tiles),
            preferred_formats: None,
            rights: None,
        }
    }

    fn open_region(
        &mut self,
        region: AbsoluteRegion,
        scaled_to: (Dimension, Dimension),
    ) -> Box<dyn ImageDecoder> {
        let info = self.info();
        let (scaled_width, scaled_height) = scaled_to;
        let kdu_region = kaduceus::Region {
            x: region.x,
            y: region.y,
            width: region.width,
            height: region.height,
        };

        let decompressor = KakaduImage::open_region(self, kdu_region, scaled_width, scaled_height);

        Box::new(decompressor)
    }
}

impl ImageDecoder for KakaduDecompressor {
    fn decode_to(&mut self, buffer: &mut BytesMut) -> bool {
        let uninit = buffer.spare_capacity_mut();

        // SAFETY: the buffer is never read by `process`
        let uninit_buf = unsafe {
            std::mem::transmute::<&mut [std::mem::MaybeUninit<u8>], &mut [u8]>(&mut uninit[..])
        };

        let region = self.process(uninit_buf).unwrap();

        unsafe {
            buffer.set_len(buffer.len() + region.width as usize * region.height as usize * 3);
        }

        info!(region=?region, "processed a region");

        region.width == 0 || region.height == 0
    }

    fn output_size(&self) -> crate::image::Dimensions {
        todo!()
    }
}

impl ImageReader for KaduceusImageReader {
    fn read<'a>(
        &'a self,
        name: Option<String>,
        location: FileOrStream,
    ) -> Pin<Box<dyn Future<Output = BoxedImage> + Send + 'a>> {
        Box::pin(async move {
            let executor = self.executor.clone();
            let context = self.context.clone();
            let span = tracing::Span::current();

            tokio::task::spawn_blocking(move || {
                span.in_scope(|| {
                    let stream = match location {
                        FileOrStream::File(file) => {
                            Box::into_pin((file.stream_factory)(&file.path))
                        }
                        FileOrStream::Stream(reader) => Box::into_pin(reader),
                    };

                    KakaduImage::new(executor, context, stream, name).boxed()
                })
            })
            .await
            .unwrap()
        })
    }
}
