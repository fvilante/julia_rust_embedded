use super::{
    decoder::{Decoder, SegmentError},
    encoder::Encoder,
    frame::Frame,
};
use crate::types::serial_connection::SerialConnection;

pub type DelayFn = fn(u64) -> ();

#[derive(Debug)]
pub enum DatalinkError {
    SegmentError(SegmentError),
    ReceptionTimeout { elapsed_time: u64 },
}

#[derive(Debug, PartialEq)]
pub struct DatalinkResult {
    pub frame: Frame,
    pub response_time_us: u64, // microseconds (aprox)
}

fn send(frame: Frame, connection: &impl SerialConnection) {
    let encoder = Encoder::new(frame);
    // transmit
    for byte in encoder {
        connection.transmit(byte);
    }
}

fn receive(
    connection: impl SerialConnection,
    timeout_us: u64,
    delay_us: DelayFn,
) -> Result<DatalinkResult, DatalinkError> {
    let mut decoder = Decoder::new();
    let mut elapsed_time: u64 = 0x00; // microseconds counter

    //receive
    loop {
        if connection.ready_to_receive() {
            let byte = connection.receive();
            let output = decoder.parse_next(byte);
            match output {
                Ok(data) => {
                    match data {
                        Some(frame) => {
                            return Ok(DatalinkResult {
                                frame,
                                response_time_us: elapsed_time,
                            });
                        }

                        None => {
                            // empty cycle => processing,
                        }
                    }
                }

                Err(e) => {
                    return Err(DatalinkError::SegmentError(e));
                }
            }
        }
        // check for timeout
        delay_us(1);
        elapsed_time += 1; //
        if elapsed_time > timeout_us {
            return Err(DatalinkError::ReceptionTimeout { elapsed_time });
        }
    }
}

pub fn transact(
    frame: Frame,
    connection: impl SerialConnection,
    timeout_us: u64,
    delay_us: DelayFn,
) -> Result<DatalinkResult, DatalinkError> {
    send(frame, &connection);
    receive(connection, timeout_us, delay_us)
}

#[cfg(test)]
mod tests {
    use crate::{
        mock::serial_connection_mock::MockedSerialConnection, protocol::prelude::StartByte,
        types::delay::delay_us,
    };

    use super::*;

    #[test]
    fn it_transact_one_frame() {
        // prepare
        let start_byte = StartByte::STX;
        let payload = [1, 2, 3, 4];
        let timeout_us: u64 = 500;
        let frame = Frame {
            start_byte,
            payload,
        };
        let connection = MockedSerialConnection::new(9600);
        let expected = DatalinkResult {
            frame,
            response_time_us: 0x00,
        };
        // act
        let actual = transact(frame, connection, timeout_us, delay_us).unwrap();
        // check
        assert_eq!(expected.frame, actual.frame);
        assert_eq!(true, actual.response_time_us < timeout_us)
    }
}
