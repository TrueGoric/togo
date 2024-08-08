use crate::storage::Flush;

pub struct ShardIdentifier(u32);

pub struct Shard<S: Flush> {
    storage: S
}