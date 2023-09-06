pub type BatchedMessage<T> = Batch<Message<T>>;

pub enum Message<T> {
    Prepare(T),
    PrepareOk,
    Commit,
    StartViewChange,
    DoViewChange,
    StartView
}

pub struct Batch<T>(Vec<T>);