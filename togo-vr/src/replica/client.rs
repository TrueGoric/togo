use std::rc::Rc;

use crate::message::ClientMessage;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ClientIdentity {}

pub struct ClientOperation<O, OR> {
    pub request_number: u64,
    pub operation: O,
    pub response: Option<OR>,
}

impl<O, OR> From<ClientMessage<O>> for ClientOperation<O, OR> {
    fn from(value: ClientMessage<O>) -> Self {
        Self {
            request_number: value.request_number,
            operation: value.request,
            response: None,
        }
    }
}

impl<O, OR> From<ClientMessage<O>> for ClientOperation<Rc<O>, OR> {
    fn from(value: ClientMessage<O>) -> Self {
        Self {
            request_number: value.request_number,
            operation: Rc::new(value.request),
            response: None,
        }
    }
}
