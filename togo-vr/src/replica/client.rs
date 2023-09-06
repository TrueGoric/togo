pub struct ClientIdentity {}

pub struct ClientOperation<O, OR> {
    request_number: u64,
    operation: O,
    response: OR,
}
