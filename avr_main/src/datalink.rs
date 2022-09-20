
use lib_1::protocol::{common::StartByte};

use crate::{
    lib_1::protocol::{
        common::Frame,
        encoder::Encoder,
        decoder::Decoder,
    }, 
    board::lcd, 
    microcontroler::serial};

fn transact(frame: Frame) -> Frame {
    serial::init(9600);
    let mut encoder = Encoder::new(StartByte::STX, frame);
    let mut decoder = Decoder::new();

    lcd::print("Send>");

    // transmit
    loop {
        let data = encoder.get_next();
        if let Some(byte) = data {
            lcd::print_u8_in_hex(byte);
            lcd::print(";");
            serial::transmit(byte);
        } else {
            break; // no more bytes to transmit
        }
    }

    lcd::clear();

    lcd::print(" Recv>Frame>");

    let mut c: u16 = 0x00;
    
    //receive
    loop {
        if serial::ready_to_receive() {
            let byte = serial::receive();
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
                        }

                        None => {
                            
                        }
                    }
                }

                Err(e) => {
                    lcd::print(e.to_string());
                }
            }
            
        }
    }

 }

pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();
    lcd::print("oi");
    // 1B 02 C1 50 61 02 1B 03 87  
    let frame = Frame(0x00, 0x50, 0x61, 0x02, );
    transact(frame);

    loop { }

}