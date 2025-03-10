use super::{FileOrStream, StorageError, StorageProvider};

pub struct OpenDalStorageProvider;

impl StorageProvider for OpenDalStorageProvider {
    fn open(id: &str) -> Result<FileOrStream, StorageError> {
        todo!()
    }
}
