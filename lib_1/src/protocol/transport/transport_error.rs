use crate::protocol::transact::DatalinkError;

use super::channel::Channel;

#[derive(Debug)]
pub enum TransportError {
    InvalidChannel(Channel),
    DatalinkError(DatalinkError),
}