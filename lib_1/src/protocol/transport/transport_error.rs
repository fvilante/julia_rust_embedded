use super::channel::Channel;
use crate::protocol::datalink::transact::DatalinkError;

#[derive(Debug)]
pub enum TransportError {
    InvalidChannel(Channel),
    DatalinkError(DatalinkError),
}
