use std::collections::VecDeque;

use super::{Log, LogError, LogResult};

pub struct MemoryLog<T> {
    offset: u64,
    data: VecDeque<T>,
}

impl<T> MemoryLog<T> {
    pub fn new() -> Self {
        Self {
            offset: 0,
            data: VecDeque::new(),
        }
    }
}

impl<T> Default for MemoryLog<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Log<T> for MemoryLog<T> {
    fn current_size(&self) -> u64 {
        self.data.len() as u64
    }

    fn current_offset(&self) -> u64 {
        self.offset
    }

    fn current_size_with_offset(&self) -> u64 {
        self.offset + self.data.len() as u64
    }

    fn get(&self, index: u64) -> LogResult<&T> {
        if index < self.offset || index > self.current_size_with_offset() {
            return Err(LogError::InvalidIndex);
        }

        let get_index = index - self.offset;

        Ok(self.data.get(get_index as usize).unwrap())
    }

    fn push(&mut self, value: T) -> LogResult<()> {
        self.data.push_back(value);

        Ok(())
    }

    fn trim_front(&mut self, first: u64) -> LogResult<()> {
        if first < self.offset || first >= self.current_size_with_offset() {
            return Err(LogError::InvalidIndex);
        }

        if first == self.offset {
            return Ok(());
        }

        let drain_to = (first - self.offset) as usize;

        self.data.drain(0..drain_to);
        self.offset = first;

        Ok(())
    }

    fn trim_end(&mut self, last: u64) -> LogResult<()> {
        if last < self.offset || last >= self.current_size_with_offset() {
            return Err(LogError::InvalidIndex);
        }

        let truncate_to = last - self.offset;

        self.data.truncate(truncate_to as usize);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::log::Log;

    use super::MemoryLog;

    #[test]
    pub fn get_retrieves_correct_element() {
        let mut log = MemoryLog::new();

        log.push(347).unwrap();
        log.push(11).unwrap();
        log.push(45).unwrap();
        log.push(125).unwrap();

        assert_eq!(log.get(0).unwrap(), &347);
        assert_eq!(log.get(2).unwrap(), &45);
        assert_eq!(log.get(log.current_size_with_offset() - 1).unwrap(), &125);

        log.trim_front(2).unwrap();

        assert_eq!(log.get(log.current_size_with_offset() - 1).unwrap(), &125);

        log.push(2233).unwrap();
        log.push(111).unwrap();

        assert_eq!(log.get(4).unwrap(), &2233);
        assert_eq!(log.get(log.current_size_with_offset() - 1).unwrap(), &111);

        log.trim_end(4).unwrap();

        assert_eq!(log.get(log.current_size_with_offset() - 1).unwrap(), &125);
    }

    #[test]
    pub fn size_is_correct_when_pushed_and_trimmed() {
        let mut log = MemoryLog::new();

        log.push(12).unwrap();
        log.push(12).unwrap();

        assert_eq!(log.current_size(), 2);
        assert_eq!(log.current_size_with_offset(), 2);

        log.push(12).unwrap();
        log.push(12).unwrap();

        assert_eq!(log.current_size(), 4);
        assert_eq!(log.current_size_with_offset(), 4);

        log.trim_front(2).unwrap();

        assert_eq!(log.current_size(), 2);
        assert_eq!(log.current_size_with_offset(), 4);

        log.push(12).unwrap();

        assert_eq!(log.current_size(), 3);
        assert_eq!(log.current_size_with_offset(), 5);

        log.trim_end(3).unwrap();

        assert_eq!(log.current_size(), 1);
        assert_eq!(log.current_size_with_offset(), 3);
    }
}
