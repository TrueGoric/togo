use std::path::Path;

use async_trait::async_trait;
use savefile::prelude::Savefile;
use sled::{Db, IVec};

use super::{Flush, Get, Snapshot, StorageError, StorageResult, Upsert};

pub struct SledStorage {
    db: Db,
}

impl Get for SledStorage {
    type ReturnValue = IVec;

    fn get<K>(&self, key: K) -> StorageResult<Option<Self::ReturnValue>>
    where
        K: AsRef<[u8]>,
    {
        self.db.get(key).map_err(|error| error.into())
    }
}

impl Upsert for SledStorage {
    fn upsert<K, V>(&self, key: K, value: V) -> StorageResult<()>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        self.db
            .insert(key, value.as_ref())
            .and(Ok(()))
            .map_err(|error| error.into())
    }
}

#[async_trait]
impl Flush for SledStorage {
    async fn flush(&self) -> StorageResult<()> {
        self.db
            .flush_async()
            .await
            .and(Ok(()))
            .map_err(|error| error.into())
    }
}

const SNAPSHOT_VERSION: u32 = 1;

impl Snapshot for SledStorage {
    fn save_snapshot(&self, path: &Path) -> StorageResult<()> {
        let snapshot: SledSnapshot = self
            .db
            .export()
            .into_iter()
            .map(|part| part.into())
            .collect::<Vec<_>>()
            .into();

        savefile::save_file(path, SNAPSHOT_VERSION, &snapshot).map_err(|error| error.into())
    }

    fn apply_snapshot(&mut self, path: &Path) -> StorageResult<()> {
        let snapshot: SledSnapshot = savefile::load_file(path, SNAPSHOT_VERSION)
            .map_err(std::convert::Into::<StorageError>::into)?;

        self.db.import(snapshot.into());

        Ok(())
    }
}

#[derive(Savefile)]
pub(crate) struct SledSnapshot(Vec<SledSnapshotPart>);

impl From<Vec<SledSnapshotPart>> for SledSnapshot {
    fn from(value: Vec<SledSnapshotPart>) -> Self {
        Self(value)
    }
}

impl From<SledSnapshot> for Vec<(Vec<u8>, Vec<u8>, std::vec::IntoIter<Vec<Vec<u8>>>)> {
    fn from(value: SledSnapshot) -> Self {
        value
            .0
            .into_iter()
            .map(|part| (part.vec_one, part.vec_two, part.data.into_iter()))
            .collect()
    }
}

#[derive(Savefile)]
pub(crate) struct SledSnapshotPart {
    pub vec_one: Vec<u8>,
    pub vec_two: Vec<u8>,
    pub data: Vec<Vec<Vec<u8>>>,
}

impl<I: Iterator<Item = Vec<Vec<u8>>>> From<(Vec<u8>, Vec<u8>, I)> for SledSnapshotPart {
    fn from(value: (Vec<u8>, Vec<u8>, I)) -> Self {
        Self {
            vec_one: value.0,
            vec_two: value.1,
            data: value.2.collect(),
        }
    }
}

impl From<Db> for SledStorage {
    fn from(value: Db) -> Self {
        SledStorage { db: value }
    }
}

impl From<sled::Error> for StorageError {
    fn from(value: sled::Error) -> Self {
        match value {
            sled::Error::CollectionNotFound(_) => StorageError::NotFound,
            sled::Error::Unsupported(message) => StorageError::Unknown(message),
            sled::Error::ReportableBug(message) => StorageError::Unknown(message),
            sled::Error::Io(error) => StorageError::Io(error),
            sled::Error::Corruption { at, bt } => StorageError::CorruptionDetected(format!(
                "Storage file is corrupted at {at:?} {bt:?}"
            )),
        }
    }
}
