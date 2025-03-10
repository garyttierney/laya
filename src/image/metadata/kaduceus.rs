use std::future::Future;

use kaduceus::{KakaduContext, KakaduImageReader};

use super::ImageMetadataResolver;
use crate::iiif::info::ImageInfo;
use crate::storage::FileOrStream;

pub struct KaduceusImageReader {
    context: KakaduContext,
}

impl KaduceusImageReader {
    pub fn new(context: KakaduContext) -> Self {
        Self { context }
    }
}

impl ImageMetadataResolver for KaduceusImageReader {
    fn info<'a>(&'a self, location: FileOrStream) -> Box<dyn Future<Output = ImageInfo> + 'a> {
        Box::new(async move {
            let stream = match location {
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
