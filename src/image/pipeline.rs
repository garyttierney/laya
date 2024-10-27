use super::{ImageMetadataResolver, ImageSourceResolver};

pub struct ImagePipelineBuilder<S: ImageSourceResolver, R: ImageMetadataResolver> {
    locator: Option<S>,
    reader: Option<R>,
}

impl<L: ImageSourceResolver, R: ImageMetadataResolver> ImagePipelineBuilder<L, R> {
    pub fn new() -> Self {
        Self { locator: None, reader: None }
    }

    pub fn with_locator(mut self, storage: L) -> Self {
        self.locator = Some(storage);
        self
    }

    pub fn with_reader(mut self, reader: R) -> Self {
        self.reader = Some(reader);
        self
    }

    pub fn build(self) -> ImagePipeline<L, R> {
        ImagePipeline {
            locator: self.locator.expect("no locator set"),
            reader: self.reader.expect("no reader set"),
        }
    }
}

pub struct ImagePipeline<L: ImageSourceResolver, R: ImageMetadataResolver> {
    locator: L,
    reader: R,
}
