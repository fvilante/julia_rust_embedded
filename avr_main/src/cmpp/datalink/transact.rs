
use lib_1::protocol::{common::StartByte, decoder::{SegmentError, SegmentResult}};
use crate::microcontroler::serial::transmit;
use crate::microcontroler::delay::delay_us;
use super::serial_connection::SerialConnection;
use super::concrete_serial::ConcreteSerialPort;

use crate::{
    lib_1::protocol::{
        common::Frame,
        encoder::Encoder,
        decoder::Decoder,
    }, 
    board::lcd, 
    microcontroler::serial
};


pub enum DatalinkError {
    SegmentError(SegmentError),
    ReceptionTimeout{elapsed_time: u64},
}

#[derive(Debug)]
pub struct DatalinkResult {
    start_byte: StartByte,
    frame: Frame,
    response_time_us: u64 // microseconds (aprox)
}

fn send(frame: Frame, connection: &impl SerialConnection)  {
    let mut encoder = Encoder::new(StartByte::STX, frame);
    // transmit
    while let Some(byte) = encoder.get_next() {
        connection.transmit(byte);
    } 
}

fn receive(connection: impl SerialConnection, timeout_us: u64) -> Result<DatalinkResult, DatalinkError> {
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
                        Some(segment) => {
                            let SegmentResult {start_byte, frame} = segment;
                            return Ok(DatalinkResult{start_byte, frame, response_time_us: elapsed_time});
                        }

                        None => {
                            
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


pub fn transact(frame: Frame, connection: impl SerialConnection, timeout_us: u64) -> Result<DatalinkResult, DatalinkError> {
    send(frame, &connection);
    receive(connection, timeout_us)
}

pub fn development_entry_point() -> ! {
    lcd::lcd_initialize();
    lcd::print("oi");
    // 1B 02 C1 50 61 02 1B 03 87  
    let frame = Frame(0x00, 0x50, 0x61, 0x02, );
    let baud_rate = 2400;
    let serial = ConcreteSerialPort::new(baud_rate);
    let result = transact(frame, serial, 200000);
    if let Ok(data) = result {
        lcd::print("FrameOk->");
        for byte in data.frame.to_array() {
            lcd::print_u8_in_hex(byte);
            lcd::print(";");
        }
    }

    loop { }

}