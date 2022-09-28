use crate::types::serial_connection::SerialConnection;
use super::{encoder::Encoder, decoder::{Decoder, SegmentError}};
use super::frame::Frame;

#[derive(Debug)]
pub enum DatalinkError {
    SegmentError(SegmentError),
    ReceptionTimeout{elapsed_time: u64},
}

#[derive(Debug, PartialEq)]
pub struct DatalinkResult {
    frame: Frame<4>,
    response_time_us: u64 // microseconds (aprox)
}

fn send(frame: Frame<4>, connection: &impl SerialConnection)  {
    let encoder = Encoder::new(frame);
    // transmit
    for byte in encoder {
        connection.transmit(byte);
    } 
}

fn receive(connection: impl SerialConnection, _timeout_us: u64) -> Result<DatalinkResult, DatalinkError> {
    let mut decoder = Decoder::new();
    let elapsed_time: u64 = 0x00; // microseconds counter
    
    //receive
    loop {
        if connection.ready_to_receive() {
            let byte = connection.receive();
            let output = decoder.parse_next(byte);
            match output {
                Ok(data) => {
                    match data {
                        Some(frame) => {
                            return Ok(DatalinkResult{frame, response_time_us: elapsed_time});
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
        //delay_us(1);
        //elapsed_time += 1; //
        //if elapsed_time > timeout_us {
            //return Err(DatalinkError::ReceptionTimeout { elapsed_time: 0 });
        //}

    }
}


pub fn transact(frame: Frame<4>, connection: impl SerialConnection, timeout_us: u64) -> Result<DatalinkResult, DatalinkError> {
    send(frame, &connection);
    receive(connection, timeout_us)
}



pub fn add(left: u8, right: u8) -> u8 {
    left + right + 2
}

#[cfg(test)]
mod tests {
    use crate::{mock::seria_connection_mock::MockedSerialConnection, protocol::common::StartByte};

    use super::*;

    #[test]
    fn it_transact() {
        // prepare
        let start_byte = StartByte::STX;
        let payload = [1,2,3,4];
        let timeout_us: u64 = 500;
        let frame = Frame { start_byte, payload };
        let connection = MockedSerialConnection::new(9600);
        let expected = DatalinkResult{frame, response_time_us: 0x00};
        // act
        let actual = transact(frame, connection, timeout_us).unwrap();
        // check
        assert_eq!(expected, actual);
    }
}
