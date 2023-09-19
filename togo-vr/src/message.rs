use crate::replica::{client::ClientIdentity, ReplicaIdentity};

pub type BatchedClusterMessage<T> = Batch<ClusterMessageEnvelope<T>>;

pub struct Batch<T>(Vec<T>);

pub struct ClusterMessageEnvelope<T> {
    pub sender: ReplicaIdentity,
    pub content: ClusterMessage<T>
}

pub enum ClusterMessage<T> {
    Prepare(PrepareMessage<T>),
    PrepareOk(PrepareOkMessage),
    Commit(CommitMessage),
    StartViewChange,
    DoViewChange,
    StartView,
}

pub struct PrepareMessage<T> {
    pub requesting_replica: ReplicaIdentity,
    pub view_number: u64,
    pub op_number: u64,
    pub commit_number: u64,
    pub client: ClientIdentity,
    pub request: T,
    pub request_number: u64,
}

pub struct PrepareOkMessage {
    pub replica: ReplicaIdentity,
    pub view_number: u64,
    pub op_number: u64,
}

pub struct CommitMessage {
    pub view_number: u64,
    pub commit_number: u64,
}

pub struct ClientMessage<T> {
    pub request_number: u64,
    pub request: T,
}
