use std::collections::BTreeSet;

use thiserror::Error;

use crate::{
    message::{BatchedClusterMessage, ClusterMessageEnvelope},
    transport::{TransportChannel, TransportResult},
};

use super::ReplicaIdentity;

pub type ClusterResult<T> = Result<T, ClusterError>;

pub struct Cluster<O, T>
where
    T: TransportChannel<ReplicaIdentity, BatchedClusterMessage<O>>,
{
    channel: T,
    replicas: BTreeSet<ReplicaIdentity>,
    current_primary: ReplicaIdentity,
    message_buffer: Vec<ClusterMessageEnvelope<O>>,
}

impl<O, T> Cluster<O, T>
where
    T: TransportChannel<ReplicaIdentity, BatchedClusterMessage<O>>,
{
    pub fn bootstrap<R: Into<BTreeSet<ReplicaIdentity>>>(
        channel: T,
        replicas: R,
    ) -> ClusterResult<Self> {
        let replicas = replicas.into();

        if replicas.len() < 3 {
            return Err(ClusterError::InsufficientReplicas);
        }

        let current_primary = *replicas.first().unwrap();

        Ok(Self {
            channel,
            replicas,
            current_primary,
            message_buffer: Vec::new(),
        })
    }

    pub fn current_primary(&self) -> ReplicaIdentity {
        self.current_primary
    }

    pub fn broadcast(&mut self, message: ClusterMessageEnvelope<O>) -> TransportResult<()> {
        todo!()
    }

    pub fn send(&mut self, recipient: ReplicaIdentity, message: ClusterMessageEnvelope<O>) -> TransportResult<()> {
        todo!()
    }

    pub async fn receive(&self) -> TransportResult<Option<ClusterMessageEnvelope<O>>> {
        todo!()
    }

    pub async fn send_bufferred_messages(&mut self) -> TransportResult<()> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum ClusterError {
    #[error("Insufficient number of replicas to establish a cluster!")]
    InsufficientReplicas,
}
