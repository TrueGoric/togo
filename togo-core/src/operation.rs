use bytes::Bytes;

pub enum Operation {
    Upsert(Bytes, Bytes),
    Delete(Bytes),
    NoOp
}