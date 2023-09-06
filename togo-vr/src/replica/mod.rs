use std::collections::BTreeMap;

use thiserror::Error;

use crate::{log::Log, message::BatchedMessage, state::StateMachine, transport::TransportChannel};

use self::{
    client::{ClientIdentity, ClientOperation},
    cluster::Cluster,
};

pub mod client;
pub mod cluster;

pub type ReplicaResult<T> = Result<T, ReplicaError>;

pub struct Replica<O, OR, T, L, S>
where
    T: TransportChannel<ReplicaIdentity, BatchedMessage<O>>,
    L: Log<O>,
    S: StateMachine<O, OR>,
{
    identity: ReplicaIdentity,
    op_log: L,
    client_log: BTreeMap<ClientIdentity, ClientOperation<O, OR>>,
    state_machine: S,
    state: ReplicaState,
    cluster: Cluster<O, T>,
}

pub struct ReplicaState {
    commit_num: u64,
    view_num: u64,
    status: ReplicaStatus,
}

#[derive(PartialEq)]
pub enum ReplicaStatus {
    Normal,
    Recovery,
    ViewChange,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ReplicaIdentity {}

impl<O, OR, T, L, S> Replica<O, OR, T, L, S>
where
    T: TransportChannel<ReplicaIdentity, BatchedMessage<O>>,
    L: Log<O>,
    S: StateMachine<O, OR>,
{
    pub fn apply_request(&mut self, client: ClientIdentity, request: O) -> ReplicaResult<()> {
        if self.cluster.current_primary() != &self.identity {
            return Err(ReplicaError::NotPrimary);
        }

        if self.state.status != ReplicaStatus::Normal {
            return Err(ReplicaError::InvalidState);
        }

        todo!()
    }
}

#[derive(Debug, Error)]
pub enum ReplicaError {
    #[error("Only a primary replica can perform this operation!")]
    NotPrimary,
    #[error("Replica isn't ready!")]
    InvalidState,
}
