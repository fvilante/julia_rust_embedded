use lib_1::protocol::common::StartByte;
use lib_1::protocol::frame::Frame;
use lib_1::protocol::transact::{transact, DatalinkError};
use crate::microcontroler::delay::delay_us;
use crate::board::lcd;
use super::concrete_serial::ConcreteSerialPort;
use lib_1::types::serial_connection::SerialConnection;




pub fn development_entry_point() -> ! {
    lcd::lcd_initialize();
    let frame = Frame{
        start_byte: StartByte::STX,
        payload: [0,0x50,0,0],
    };
    let baud_rate = 2400;
    let connection = ConcreteSerialPort::new(baud_rate);
    let timeout_us = 200*1000;
    let response = transact(frame, connection, timeout_us, delay_us);
    match response {
        Err(e) => {
            lcd::print("Datalink Error: ");
            match e {
                DatalinkError::SegmentError(e0) => {
                    lcd::print("SegmentError");
                    lcd::print(e0.to_string());
                }
                DatalinkError::ReceptionTimeout { elapsed_time } => {
                    lcd::print("TimeoutError");
                    lcd::print_u16_in_hex(elapsed_time.try_into().unwrap());
                }
            }
        }
        Ok(_frame) => {
            lcd::print("Successful datalink transaction!")
        }
    }

    loop {}
}