use thiserror::Error;

pub type StateResult<T> = Result<T, StateError>;

pub trait StateMachine<O, OR> {
    fn apply_operations(&self, operations: &[O]) -> StateResult<Vec<OR>>;
}

#[derive(Debug, Error)]
pub enum StateError {}
