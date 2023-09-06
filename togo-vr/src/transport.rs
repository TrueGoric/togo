use async_trait::async_trait;
use thiserror::Error;

pub type TransportResult<T> = Result<T, TransportError>;

#[async_trait]
pub trait TransportChannel<I, T> {
    async fn send(&self, recipient: I, message: T) -> TransportResult<()>;
    async fn receive(&self) -> TransportResult<Option<T>>;
}

#[derive(Debug, Error)]
pub enum TransportError {

}