use std::future::Future;

use crate::iiif::info::ImageInfo;

use super::{Image, FileOrStream};

pub mod kaduceus;

pub trait ImageReader {
    type ImageType: Image;

    fn read<'a>(&'a self, location: FileOrStream) -> Box<dyn Future<Output = Self::ImageType> +'a>;
}