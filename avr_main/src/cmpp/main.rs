use heapless::Deque;
use lib_1::protocol::datalink::{
    decoder::{Decoder, DecodingError},
    frame::Frame,
    prelude::StartByte,
    transact::{transact, DatalinkError},
};

use crate::{
    board::lcd,
    microcontroler::{delay::delay_us, serial},
};

use super::datalink::concrete_serial::ConcreteSerialPort;
use lib_1::types::serial_connection::SerialConnection;

/// Helper to just clamp values
fn delay_us_helper(time_us: u64) {
    let time_us_clamped = time_us.clamp(u32::MIN as u64, u32::MAX as u64);
    delay_us(time_us_clamped)
}

fn test_cmpp() {
    let frame = Frame::make_master_block([0, 0x50, 0, 0].into());
    let connection = ConcreteSerialPort::new(2400);
    const timeout_us: u64 = 200 * 1000;
    let response = transact(frame, connection, timeout_us, delay_us_helper);
    match response {
        Ok(_response) => lcd::print("Response Ok"),
        Err(error) => {
            lcd::print("Response Err");
            match error {
                DatalinkError::DecodingError(error) => {
                    lcd::print("DecodingError");

                    match error {
                        DecodingError::InvalidStartByte(start_byte) => {
                            lcd::print("InvalidStartByte=");
                            lcd::print_u8_in_hex(start_byte)
                        }
                        DecodingError::BufferOverFlow => {
                            lcd::print("BufferOverFlow");
                        }
                        DecodingError::ExpectedEtxOrEscDupButFoundOtherThing(_) => {
                            lcd::print("ExpectedEtxOrEscDupButFoundOtherThing");
                        }
                        DecodingError::ChecksumIsEscButNotDuplicated(_) => {
                            lcd::print("ChecksumIsEscButNotDuplicated");
                        }
                        DecodingError::InvalidChecksum {
                            expected: _,
                            received: _,
                        } => {
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
}

fn teste_dequeue() {
    let mut queue: Deque<u8, 3> = Deque::new();

    //queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);
    lcd::print_u8_in_hex(queue.pop_front().unwrap());
    match queue.push_back(4) {
        Ok(_) => lcd::print("4o elemento OK"),
        Err(_) => lcd::print("4o elemento PROBLEMA"),
    }
    // queue.push_front(20);
    lcd::print_u8_in_hex(queue.pop_front().unwrap());
    lcd::print_u8_in_hex(queue.pop_front().unwrap());
    lcd::print_u8_in_hex(queue.pop_front().unwrap());
}

pub fn development_entry_point() -> ! {
    lcd::lcd_initialize();
    lcd::print("Juca kifuri");

    let frame = Frame::new(StartByte::STX, [0, 0x50, 0, 0].into());

    serial::init(9600);

    lcd::print("(A);");
    for byte in frame.encode() {
        loop {
            if let Ok(_) = serial::try_transmit(byte) {
                break;
            }
        }
    }
    lcd::print("(B);");

    let _count: u16 = 0;

    let mut decoder = Decoder::new();

    loop {
        if let Some(byte) = serial::try_receive() {
            match decoder.parse_next(byte) {
                Ok(res) => {
                    match res {
                        Some(_frame) => {
                            lcd::print("Success");
                        }
                        None => {
                            // processing input
                        }
                    }
                }
                Err(_error) => {
                    lcd::print("Error");
                }
            }
        }
    }

    //crate::microcontroler::serial::development_entry_point();

    //test_cmpp();

    loop {}
}
