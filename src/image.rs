mod pipeline;
use std::ops::{Deref, DerefMut};

pub use pipeline::{ImagePipeline, ImagePipelineBuilder};

pub mod codec;
pub use codec::ImageReader;

use crate::iiif::info::ImageInfo;

pub trait Image {
    fn boxed(self) -> BoxedImage
    where
        Self: Sized + Send + 'static,
    {
        BoxedImage(Box::new(self))
    }

    fn info(&mut self) -> ImageInfo;
}

pub struct BoxedImage(Box<dyn Image + Send>);

impl Deref for BoxedImage {
    type Target = Box<dyn Image + Send>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxedImage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Image for BoxedImage {
    fn info(&mut self) -> ImageInfo {
        self.0.info()
    }
}
