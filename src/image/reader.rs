use std::future::Future;

use super::{FileOrStream, Image};
use crate::iiif::info::ImageInfo;

pub mod kaduceus;

pub trait ImageReader {
    type ImageType: Image;

    fn read<'a>(&'a self, location: FileOrStream)
        -> Box<dyn Future<Output = Self::ImageType> + 'a>;
}
