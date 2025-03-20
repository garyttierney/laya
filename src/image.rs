use std::ops::{Deref, DerefMut};

use bytes::{Bytes, BytesMut};
use futures::Stream;
use mediatype::MediaTypeBuf;

pub mod codec;
pub mod info;
pub mod transcoding;

pub use codec::ImageReader;
use info::ImageInfo;

use crate::iiif::{Dimension, Region};

pub type Dimensions = (Dimension, Dimension);

#[derive(Clone, Copy)]
pub struct AbsoluteRegion {
    x: Dimension,
    y: Dimension,
    width: Dimension,
    height: Dimension,
}

/// An asynchronous sequential stream of encoded image data and the associated
/// [mediatype::MediaType]
pub struct ImageStream {
    pub media_type: MediaTypeBuf,
    pub data: Box<dyn Stream<Item = Result<Bytes, std::io::Error>> + Send + Sync + Unpin>,
}

pub trait ImageDecoder {
    fn output_size(&self) -> Dimensions;
    fn decode_to(&mut self, buffer: &mut BytesMut) -> bool;
}

pub trait Image {
    fn boxed(self) -> BoxedImage
    where
        Self: Sized + Send + 'static,
    {
        BoxedImage(Box::new(self))
    }

    fn info(&mut self) -> ImageInfo;

    fn open_region(
        &mut self,
        region: AbsoluteRegion,
        scaled_to: Dimensions,
    ) -> Box<dyn ImageDecoder>;
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

    fn open_region(
        &mut self,
        region: AbsoluteRegion,
        scaled_to: (Dimension, Dimension),
    ) -> Box<dyn ImageDecoder> {
        self.0.open_region(region, scaled_to)
    }
}
