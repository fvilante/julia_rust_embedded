use crate::types::serial_connection::SerialConnection;
use crate::protocol::transact::DelayFn;
use crate::protocol::transact::DatalinkError;
use crate::protocol::transact::DatalinkResult;
use crate::protocol::transact::transact;
use super::master_packet::make_frame;
use super::master_packet::MasterPacket;



pub fn transact_packet(master_packet: MasterPacket, connection: impl SerialConnection, timeout_us: u64, delay_us: DelayFn) -> Result<DatalinkResult, DatalinkError> {
    let frame = make_frame(master_packet);
    let response = transact(frame, connection, timeout_us, delay_us);
    response
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::serial_connection_mock::MockedSerialConnection;
    use crate::protocol::transport::master_packet::MasterPacket;
    use crate::protocol::transact::DatalinkResult;
    use crate::protocol::frame::Frame;
    use crate::protocol::common::StartByte;
    use crate::types::delay::delay_us;

    #[test]
    fn it_transact_one_packet() {
         // prepare
         let start_byte = StartByte::STX;
         let payload = [0,0,0,0];
         let timeout_us: u64 = 500;
         let frame = Frame { start_byte, payload };
         let connection = MockedSerialConnection::new(9600);
         let expected = DatalinkResult{frame, response_time_us: 0x00};
         let channel = 0x00;
         let waddr = 0x00;
         let master_packet = MasterPacket::GetWord { channel, waddr };
         // act
         let actual = transact_packet(master_packet, connection, timeout_us, delay_us).unwrap();
         // check
         assert_eq!(expected.frame, actual.frame);
         assert_eq!(true, actual.response_time_us < timeout_us)
    }
}