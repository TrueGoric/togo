use async_trait::async_trait;
use thiserror::Error;

pub type StateResult<T> = Result<T, StateError>;

#[async_trait]
pub trait StateMachine<O, OR> {
    async fn apply_operations(&self, operations: &[O]) -> StateResult<Vec<OR>>;
}

#[derive(Debug, Error)]
pub enum StateError {

}