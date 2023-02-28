use super::channel::Channel;
use super::master_packet::make_frame;
use super::master_packet::CmppMessage;
use super::transport_error::TransportError;
use crate::protocol::datalink::transact::transact;
use crate::protocol::datalink::transact::DatalinkResult;
use crate::protocol::datalink::transact::DelayFn;
use crate::types::serial_connection::SerialConnection;

pub fn transact_packet(
    channel: Channel,
    cmpp_mmesage: CmppMessage,
    connection: impl SerialConnection,
    timeout_us: u64,
    delay_us: DelayFn,
) -> Result<DatalinkResult, TransportError> {
    let normalize_error = |datalink_error| TransportError::DatalinkError(datalink_error);
    let frame = make_frame(channel, cmpp_mmesage);
    let response = transact(frame, connection, timeout_us, delay_us).map_err(normalize_error);
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::serial_connection_mock::MockedSerialConnection;
    use crate::protocol::datalink::frame::{Frame, Payload};
    use crate::protocol::datalink::prelude::StartByte;
    use crate::protocol::datalink::transact::DatalinkResult;
    use crate::protocol::transport::master_packet::CmppMessage;
    use crate::types::delay::delay_us;

    #[test]
    fn it_transact_one_packet() {
        // prepare
        let start_byte = StartByte::STX;
        let payload = Payload::from_array([0, 0, 0, 0]);
        let timeout_us: u64 = 500;
        let connection = MockedSerialConnection::new(9600);
        let channel = Channel::from_u8(0x00).unwrap();
        let waddr = 0x00;
        let cmpp_message = CmppMessage::GetWord { waddr };
        let frame = Frame {
            start_byte,
            payload,
        };
        let expected = DatalinkResult {
            frame,
            response_time_us: 0x00,
        };
        // act
        let actual =
            transact_packet(channel, cmpp_message, connection, timeout_us, delay_us).unwrap();
        // check
        assert_eq!(expected.frame, actual.frame);
        assert_eq!(true, actual.response_time_us < timeout_us)
    }
}
