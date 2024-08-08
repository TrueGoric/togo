use thiserror::Error;

use crate::storage::StorageError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Storage error: {}", .0)]
    StorageError(StorageError),
}

impl From<StorageError> for Error {
    fn from(value: StorageError) -> Self {
        Self::StorageError(value)
    }
}
