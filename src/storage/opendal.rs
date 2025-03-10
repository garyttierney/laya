use opendal::Operator;

use super::{FileOrStream, StorageError, StorageProvider};

pub struct OpenDALStorageProvider;

impl StorageProvider for OpenDALStorageProvider {
    fn open(id: &str) -> Result<FileOrStream, StorageError> {}
}
