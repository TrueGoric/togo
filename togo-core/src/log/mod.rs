use std::ops::Range;

pub mod blocklog;
mod extensions;

pub use extensions::*;
use thiserror::Error;

pub type LogResult<T> = Result<T, LogError>;
 
pub trait Get<T> {
    fn get(&self, index: u64) -> &T;
    fn get_cloned(&self, index: u64) -> T;
}

pub trait GetRange<T> {
    fn get_range(&self, range: Range<u64>) -> &[T];
    fn get_range_cloned(&self, range: Range<u64>) -> Vec<T>;
}

pub trait AsyncPush<T> {
    fn push(&mut self, value: T) -> LogResult<()>;
}

pub trait Trim {
    fn trim_front(&mut self, first: u64) -> LogResult<()>;
    fn trim_end(&mut self, last: u64) -> LogResult<()>;
}

#[derive(Debug, Error)]
pub enum LogError {

}