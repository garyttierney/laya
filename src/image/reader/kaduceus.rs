use std::future::Future;

use kaduceus::{KakaduContext, KakaduImageReader};

use super::ImageReader;
use crate::iiif::info::ImageInfo;
use crate::image::{FileOrStream, Image, ImageMetadata};

pub struct KaduceusImageReader {
    context: KakaduContext,
}

pub struct KaduceusImage {
    reader: KakaduImageReader,
}

impl Image for KaduceusImage {
    fn dimensions(&mut self) -> (crate::iiif::Dimension, crate::iiif::Dimension) {
        todo!()
    }
}

impl ImageReader for KaduceusImageReader {
    type ImageType = KaduceusImage;

    fn read<'a>(
        &'a self,
        location: FileOrStream,
    ) -> Box<dyn Future<Output = Self::ImageType> + 'a> {
        Box::new(async move {
            let mut stream = match location {
                FileOrStream::File(path) => {
                    todo!()
                }
                FileOrStream::Stream(reader) => Box::into_pin(reader),
            };

            let mut reader = KakaduImageReader::new(self.context.clone(), stream, None);

            KaduceusImage { reader }
        })
    }
}

impl KaduceusImageReader {
    pub fn new(context: KakaduContext) -> Self {
        Self { context }
    }
}

impl ImageMetadata for KaduceusImageReader {
    fn info<'a>(&'a self, location: FileOrStream) -> Box<dyn Future<Output = ImageInfo> + 'a> {
        Box::new(async move {
            let mut stream = match location {
                FileOrStream::File(path) => {
                    todo!()
                }
                FileOrStream::Stream(reader) => Box::into_pin(reader),
            };

            let mut reader = KakaduImageReader::new(self.context.clone(), stream, None);
            let kdu_info = reader.info();

            todo!()
        })
    }
}
