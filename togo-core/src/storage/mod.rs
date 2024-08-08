use std::path::Path;

use async_trait::async_trait;
use savefile::SavefileError;
use thiserror::Error;

pub mod sled;

pub type StorageResult<T> = Result<T, StorageError>;

pub trait Get {
    type ReturnValue: AsRef<[u8]>;

    fn get<K>(&self, key: K) -> StorageResult<Option<Self::ReturnValue>>
    where
        K: AsRef<[u8]>;
}

pub trait Upsert {
    fn upsert<K, V>(&self, key: K, value: V) -> StorageResult<()>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>;
}

#[async_trait]
pub trait Flush {
    async fn flush(&self) -> StorageResult<()>;
}

#[async_trait]
pub trait Snapshot {
    fn save_snapshot(&self, path: &Path) -> StorageResult<()>;
    fn apply_snapshot(&mut self, path: &Path) -> StorageResult<()>;
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IO error: {}", .0)]
    Io(std::io::Error),
    #[error("Couldn't find the key!")]
    NotFound,
    #[error("{}", .0)]
    Unknown(String),
    #[error("Storage/snapshot corrupted! {}", .0)]
    CorruptionDetected(String),
}

impl From<SavefileError> for StorageError {
    fn from(value: SavefileError) -> Self {
        match value {
            SavefileError::IncompatibleSchema { message } => {
                StorageError::CorruptionDetected(message)
            }
            SavefileError::IOError { io_error } => StorageError::Io(io_error),
            SavefileError::InvalidUtf8 { msg } => StorageError::CorruptionDetected(format!(
                "Invalid UTF-8 char in snapshot file! {msg}"
            )),
            SavefileError::MemoryAllocationLayoutError => StorageError::CorruptionDetected(
                "Snapshot file invalid memory allocation layout!".into(),
            ),
            SavefileError::ArrayvecCapacityError { msg } => StorageError::CorruptionDetected(
                format!("Snapshot file invalid arrayvec capacity! {msg}"),
            ),
            SavefileError::ShortRead => {
                StorageError::CorruptionDetected("Snapshot file ended unexpectedly!".into())
            }
            SavefileError::CryptographyError => {
                StorageError::CorruptionDetected("Snapshot file checksum mismatch!".into())
            }
            SavefileError::SizeOverflow => {
                StorageError::CorruptionDetected("Size overflow in snapshot file!".into())
            }
            SavefileError::WrongVersion { msg } => {
                StorageError::CorruptionDetected(format!("Invalid snapshot version! {msg}"))
            }
            SavefileError::GeneralError { msg } => StorageError::Unknown(msg),
            SavefileError::PoisonedMutex => {
                StorageError::Unknown("Poisoned mutex when traversing snapshot to be saved!".into())
            }
            SavefileError::CompressionSupportNotCompiledIn => {
                StorageError::Unknown("Snapshot compression not supported!".into())
            }
            SavefileError::InvalidChar => {
                StorageError::CorruptionDetected("Invalid char in snapshot file!".into())
            }
            _ => StorageError::Unknown("Unknown snapshot error!".into()),
        }
    }
}
