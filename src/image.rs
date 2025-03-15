use std::ops::{Deref, DerefMut};

use bytes::Bytes;
use futures::Stream;
use mediatype::MediaTypeBuf;

pub mod codec;
pub use codec::ImageReader;

use crate::iiif::Region;
use crate::iiif::info::ImageInfo;

/// An asynchronous sequential stream of encoded image data and the associated
/// [mediatype::MediaType]
pub struct ImageStream {
    pub media_type: MediaTypeBuf,
    pub data: Box<dyn Stream<Item = Bytes> + Send + Sync + Unpin>,
}

pub trait ImageDecoder {
    fn decode(self) -> ImageStream;
}

pub trait Image {
    fn boxed(self) -> BoxedImage
    where
        Self: Sized + Send + 'static,
    {
        BoxedImage(Box::new(self))
    }

    fn info(&mut self) -> ImageInfo;

    fn open_region(&mut self, region: Region) -> Box<dyn ImageDecoder>;
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

    fn open_region(&mut self, region: Region) -> Box<dyn ImageDecoder> {
        self.0.open_region(region)
    }
}
