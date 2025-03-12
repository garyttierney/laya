use std::future::Future;
use std::pin::Pin;

use kaduceus::{KakaduContext, KakaduImage};

use super::ImageReader;
use crate::iiif::info::{ImageInfo, PreferredSize, Tile};
use crate::iiif::Size;
use crate::image::{BoxedImage, Image};
use crate::storage::FileOrStream;

pub struct KaduceusImageReader {
    context: KakaduContext,
}

impl KaduceusImageReader {
    pub fn new(context: KakaduContext) -> Self {
        Self { context }
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
}

impl ImageReader for KaduceusImageReader {
    fn read<'a>(
        &'a self,
        location: FileOrStream,
    ) -> Pin<Box<dyn Future<Output = BoxedImage> + Send + 'a>> {
        Box::pin(async move {
            let stream = match location {
                FileOrStream::File(path) => {
                    todo!()
                }
                FileOrStream::Stream(reader) => Box::into_pin(reader),
            };

            KakaduImage::new(self.context.clone(), stream, None).boxed()
        })
    }
}
