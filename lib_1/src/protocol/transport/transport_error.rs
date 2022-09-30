use super::channel::Channel;

#[derive(Debug)]
pub enum TransportError {
    InvalidChannel(Channel),
}