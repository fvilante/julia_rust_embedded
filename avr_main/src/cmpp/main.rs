use lib_1::protocol::{
    decoder::SegmentError,
    frame::Frame,
    prelude::StartByte,
    transact::{transact, DatalinkError},
};

use crate::{board::lcd, microcontroler::delay::delay_us};

use super::datalink::concrete_serial::ConcreteSerialPort;
use lib_1::types::serial_connection::SerialConnection;

/// Helper to just clamp values
fn delay_us_helper(time_us: u64) {
    let time_us_clamped = time_us.clamp(u32::MIN as u64, u32::MAX as u64);
    delay_us(time_us_clamped)
}

pub fn development_entry_point() -> ! {
    lcd::lcd_initialize();
    lcd::print("Juca kifuri");

    let frame = Frame::make_master_block([0, 0x50, 0, 0]);
    let connection = ConcreteSerialPort::new(2400);
    const timeout_us: u64 = 200 * 1000;
    let response = transact(frame, connection, timeout_us, delay_us_helper);
    match response {
        Ok(_response) => lcd::print("Response Ok"),
        Err(error) => {
            lcd::print("Response Err");
            match error {
                DatalinkError::SegmentError(error) => {
                    lcd::print("SegmentError");

                    match error {
                        SegmentError::InvalidStartByte(start_byte) => {
                            lcd::print("InvalidStartByte=");
                            lcd::print_u8_in_hex(start_byte)
                        }
                        SegmentError::BufferOverFlow => {
                            lcd::print("BufferOverFlow");
                        }
                        SegmentError::ExpectedEtxOrEscDupButFoundOtherThing(_) => {
                            lcd::print("ExpectedEtxOrEscDupButFoundOtherThing");
                        }
                        SegmentError::ChecksumIsEscButNotDuplicated(_) => {
                            lcd::print("ChecksumIsEscButNotDuplicated");
                        }
                        SegmentError::InvalidChecksum { expected, received } => {
                            lcd::print("InvalidChecksum");
                        }
                    }
                }
                DatalinkError::ReceptionTimeout { elapsed_time } => {
                    lcd::print("Timeout=");
                    let _e = elapsed_time / 1000; //to milisec
                }
            }
        }
    }

    loop {}
}
