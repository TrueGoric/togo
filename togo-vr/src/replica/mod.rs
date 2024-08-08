use std::{collections::BTreeMap, ops::Deref, rc::Rc, marker::PhantomData};

use thiserror::Error;

use crate::{
    log::Log,
    message::{
        BatchedClusterMessage, ClientMessage, ClusterMessage, ClusterMessageEnvelope,
        PrepareMessage, PrepareOkMessage, CommitMessage,
    },
    state::StateMachine,
    transport::{TransportChannel, TransportError},
};

use self::{
    client::{ClientIdentity, ClientOperation, RequestNumber},
    cluster::Cluster,
};

pub mod client;
pub mod cluster;

pub type ReplicaResult<T> = Result<T, ReplicaError>;

pub struct Replica<O, OR, T, L, S>
where
    T: TransportChannel<ReplicaIdentity, BatchedClusterMessage<O>>,
    L: Log<ClientOperation<O, OR>>,
    S: StateMachine<O, OR>,
{
    identity: ReplicaIdentity,
    op_log: L,
    client_log: BTreeMap<ClientIdentity, RequestNumber>,
    state_machine: S,
    state: ReplicaState,
    cluster: Cluster<O, T>,
    phantom_operation_result: PhantomData<OR>
}

pub struct ReplicaState {
    commit_number: u64,
    view_number: u64,
    ticks_since_last_commit: u64,
    status: ReplicaStatus,
}

#[derive(PartialEq)]
pub enum ReplicaStatus {
    Normal,
    Recovery,
    ViewChange,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ReplicaIdentity {}

impl<O, OR, T, L, S> Replica<O, OR, T, L, S>
where
    O: ToOwned<Owned = O>,
    OR: ToOwned<Owned = O>,
    T: TransportChannel<ReplicaIdentity, BatchedClusterMessage<O>>,
    L: Log<Rc<O>>,
    S: StateMachine<O, OR>,
{
    pub fn apply_request(
        &mut self,
        client: ClientIdentity,
        request: ClientMessage<O>,
    ) -> ReplicaResult<()> {
        if self.cluster.current_primary() != self.identity {
            return Err(ReplicaError::NotPrimary);
        }

        if self.state.status != ReplicaStatus::Normal {
            return Err(ReplicaError::InvalidState);
        }

        if let Some(last_request) = self.client_log.get_mut(&client) {
            if last_request.request_number > request.request_number {
                return Err(ReplicaError::UnexpectedRequestNumber {
                    request_number: last_request.request_number,
                    replica_number: request.request_number,
                });
            }

            if last_request.request_number == request.request_number {
                // TODO: we shouldn't assume that it's the same request even if request_number matches
                return Ok(());
            }
        }

        let new_operation: ClientOperation<Rc<O>, OR> = request.into();

        let prepare_message = self.new_message(ClusterMessage::Prepare(PrepareMessage {
            requesting_replica: self.identity,
            view_number: self.state.view_number,
            op_number: self.op_log.current_size_with_offset(),
            commit_number: self.state.commit_number,
            client,
            request: new_operation.operation.deref().to_owned(),
            request_number: new_operation.request_number,
        }));

        self.op_log.push(new_operation.operation.clone());
        self.client_log.insert(client, new_operation);

        self.cluster
            .broadcast(prepare_message)
            .map_err(ReplicaError::TransportIssue)
    }

    pub fn apply_prepare(&mut self, message: PrepareMessage<O>) -> ReplicaResult<()> {
        if self.cluster.current_primary() == self.identity {
            return Err(ReplicaError::NotForPrimary);
        }

        if message.view_number > self.state.view_number {
            // TODO: go into recovery and initiate a state transfer
            todo!()
        }

        if message.view_number < self.state.view_number {
            // TODO: drop the message and notify the replica that it's lagging behind
            todo!()
        }

        let op_number = self.op_log.current_size_with_offset();

        // this replica is up to date, nod politely
        if message.op_number <= op_number {
            self.cluster
                .send(message.requesting_replica, self.new_prepare_ok_message())
                .map_err(ReplicaError::TransportIssue)?;

            return Ok(());
        }

        if message.op_number > op_number + 1 {
            // TODO: go into recovery and initiate a state transfer
            todo!()
        }

        assert_eq!(message.op_number, op_number + 1);

        let new_operation = ClientOperation {
            request_number: message.request_number,
            operation: Rc::new(message.request),
            response: None,
        };

        self.op_log.push(new_operation.operation.clone());
        self.client_log.insert(message.client, new_operation);

        self.cluster
            .send(message.requesting_replica, self.new_prepare_ok_message())
            .map_err(ReplicaError::TransportIssue)?;

        Ok(())
    }

    pub fn apply_commit(&self, message: CommitMessage) -> ReplicaResult<()>{
        if self.cluster.current_primary() == self.identity {
            return Err(ReplicaError::NotForPrimary);
        }

        todo!()
    }

    pub fn advance_time(&mut self) {
        todo!()
    }

    fn new_message(&self, content: ClusterMessage<O>) -> ClusterMessageEnvelope<O> {
        ClusterMessageEnvelope {
            sender: self.identity,
            content,
        }
    }

    fn new_prepare_ok_message(&self) -> ClusterMessageEnvelope<O> {
        self.new_message(ClusterMessage::PrepareOk(PrepareOkMessage {
            replica: self.identity,
            view_number: self.state.view_number,
            op_number: self.op_log.current_size_with_offset(),
        }))
    }

    fn commit(&mut self, up_to_operation: u64) {

    }
}

#[derive(Debug, Error)]
pub enum ReplicaError {
    #[error("Only a non-primary replica can perform this operation!")]
    NotForPrimary,
    #[error("Only a primary replica can perform this operation!")]
    NotPrimary,
    #[error("Replica isn't ready!")]
    InvalidState,
    #[error("Client message contained an unexpected request number! (request: {}, replica: {})", .request_number, .replica_number)]
    UnexpectedRequestNumber {
        request_number: u64,
        replica_number: u64,
    },
    #[error("An error occurred when attempting to send a message! {}", .0)]
    TransportIssue(TransportError),
}
