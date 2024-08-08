use std::path::PathBuf;

pub struct BlockLog<T> {

}

pub(crate) struct Block<T> {
    pub path: PathBuf,
    pub start_id: u64,
    pub data: Vec<T>
}