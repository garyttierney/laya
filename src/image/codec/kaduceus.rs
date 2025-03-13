use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use kaduceus::{KakaduContext, KakaduDecompressor, KakaduImage};
use mediatype::names::{IMAGE, JP2};
use mediatype::MediaTypeBuf;
use tokio::runtime::{Builder, Runtime};

use super::ImageReader;
use crate::iiif::info::{ImageInfo, PreferredSize, Tile};
use crate::iiif::Region;
use crate::image::{BoxedImage, Image, ImageDecoder, ImageStream};
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

    fn open_region(&mut self, region: Region) -> Box<dyn ImageDecoder> {
        todo!()
    }
}

pub struct KaduceusImageDecoder {}

impl ImageDecoder for KaduceusImageDecoder {
    fn decode(self) -> ImageStream {
        let media_type = MediaTypeBuf::new(IMAGE, JP2);

        ImageStream { media_type, data: todo!() }
    }
}

impl ImageReader for KaduceusImageReader {
    fn read<'a>(
        &'a self,
        location: FileOrStream,
    ) -> Pin<Box<dyn Future<Output = BoxedImage> + Send + 'a>> {
        Box::pin(async move {
            let executor = self.executor.clone();
            let context = self.context.clone();

            tokio::task::spawn_blocking(move || {
                let stream = match location {
                    FileOrStream::File(path) => {
                        todo!()
                    }
                    FileOrStream::Stream(reader) => Box::into_pin(reader),
                };

                KakaduImage::new(executor, context, stream, None).boxed()
            })
            .await
            .unwrap()
        })
    }
}
