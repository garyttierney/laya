use std::future::Future;
use std::pin::Pin;

use kaduceus::{KakaduContext, KakaduImage};

use super::ImageReader;
use crate::iiif::info::ImageInfo;
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

        ImageInfo {
            width: info.width,
            height: info.height,
            max_width: Some(info.width),
            max_height: Some(info.height),
            max_area: None,
            sizes: None,
            tiles: None,
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
