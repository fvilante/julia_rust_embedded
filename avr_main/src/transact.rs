
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

enum ReceptionError {
    SegmentError(SegmentError),
    ReceptionTimeout{elapsed_time: u64},
}

struct TransactResult {
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



fn transact(frame: Frame, connection: impl SerialConnection, timeout_us: u64) -> Result<TransactResult, ReceptionError> {

   
    let mut decoder = Decoder::new();

    // transmit
    send(frame, &connection);

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
                            return Ok(TransactResult{frame, response_time_us: elapsed_time});
                        }

                        None => {
                            
                        }
                    }
                }

                Err(e) => {
                    lcd::print(e.to_string());
                    return Err(ReceptionError::SegmentError(e));
                }
            }
            
        }
        delay_us(1);
        elapsed_time += 1; //
        if elapsed_time > timeout_us {
            return Err(ReceptionError::ReceptionTimeout { elapsed_time });
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