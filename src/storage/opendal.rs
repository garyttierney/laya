use super::{FileOrStream, StorageError, StorageProvider};

pub struct OpenDALStorageProvider;

impl StorageProvider for OpenDALStorageProvider {
    fn open(_id: &str) -> Result<FileOrStream, StorageError> {
        unimplemented!()
    }
}
