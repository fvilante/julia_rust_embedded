
use lib_1::protocol::{common::StartByte, decoder::SegmentError};
use super::microcontroler::serial::transmit;
use crate::microcontroler::delay::delay_us;

use crate::{
    lib_1::protocol::{
        common::Frame,
        encoder::Encoder,
        decoder::Decoder,
    }, 
    board::lcd, 
    microcontroler::serial
};

trait SerialConnection {
    fn new(baud_rate: u32) -> Self;
    fn transmit(&self, byte: u8);
    fn ready_to_receive(&self) -> bool;
    fn receive(&self) -> u8;
}

pub struct ConcreteSerialPort {
    baud_rate: u32,
}

impl SerialConnection for ConcreteSerialPort {

    fn new(baud_rate: u32) -> Self {
        serial::init(baud_rate);
        Self { 
            baud_rate,
        }
    }

    fn transmit(&self, byte: u8) {
        serial::transmit(byte)
    }
    fn ready_to_receive(&self) -> bool {
        serial::ready_to_receive()
    }
    fn receive(&self) -> u8 {
        serial::receive()
    }

}

enum TransactionError {
    SegmentError(SegmentError),
    ReceptionTimeout{elapsed_time: u64},
}



fn transact(frame: Frame, connection: impl SerialConnection, timeout_us: u64) -> Result<Frame, TransactionError> {

    let mut encoder = Encoder::new(StartByte::STX, frame);
    let mut decoder = Decoder::new();

    lcd::print("Send>");

    // transmit
    loop {
        let data = encoder.get_next();
        if let Some(byte) = data {
            lcd::print_u8_in_hex(byte);
            lcd::print(";");
            connection.transmit(byte);
        } else {
            break; // no more bytes to transmit
        }
    }

    lcd::clear();

    lcd::print(" Recv>Frame>");

    let mut elapsed_time: u64 = 0x00; // microseconds counter
    
    //receive
    loop {
        if connection.ready_to_receive() {
            let byte = connection.receive();
            lcd::print_u8_in_hex(byte);
            let output = decoder.parse_next(byte);
            match output {
                Ok(data) => {
                    match data {
                        Some(frame) => {
                            let Frame(d0,d1,d2,d3) = frame;
                            lcd::print("frame_ok=");
                            lcd::print_u8_in_hex(d0);
                            lcd::print_u8_in_hex(d1);
                            lcd::print_u8_in_hex(d2);
                            lcd::print_u8_in_hex(d3);
                            return Ok(frame);
                        }

                        None => {
                            
                        }
                    }
                }

                Err(e) => {
                    lcd::print(e.to_string());
                    return Err(TransactionError::SegmentError(e));
                }
            }
            
        }
        delay_us(1);
        elapsed_time += 1; //
        if elapsed_time > timeout_us {
            lcd::print("Timeout");
            return Err(TransactionError::ReceptionTimeout { elapsed_time });
        }

    }

 }

pub fn development_entry_point() -> ! {
    lcd::lcd_initialize();
    lcd::print("oi");
    // 1B 02 C1 50 61 02 1B 03 87  
    let frame = Frame(0x00, 0x50, 0x61, 0x02, );
    let baud_rate = 2400;
    let serial = ConcreteSerialPort::new(baud_rate);
    transact(frame, serial, 200000);

    loop { }

}