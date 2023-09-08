use thiserror::Error;

pub mod memory;

pub type LogResult<T> = Result<T, LogError>;

pub trait Log<T> {
    fn current_size(&self) -> u64;
    fn current_offset(&self) -> u64;
    fn current_size_with_offset(&self) -> u64;
    fn get(&self, index: u64) -> LogResult<&T>;
    fn push(&mut self, value: T);
    fn trim_front(&mut self, first: u64) -> LogResult<()>;
    fn trim_end(&mut self, last: u64) -> LogResult<()>;
}

#[derive(Debug, Error)]
pub enum LogError {
    #[error("Invalid index was supplied!")]
    InvalidIndex
}