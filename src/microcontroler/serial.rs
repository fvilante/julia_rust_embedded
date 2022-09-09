
use ruduino::legacy::serial;

const BAUD: u32 = 9600;
const UBRR: u16 = (ruduino::config::CPU_FREQUENCY_HZ / 16 / BAUD - 1) as u16;

use crate::{
    board::lcd, 
    common::convert_u8_to_str_hex,
};

use super::delay::delay_ms;

pub fn serial_development_entry_point() -> ! {

    lcd::lcd_initialize();
    lcd::print("Inicializado serial");

    serial::Serial::new(UBRR)
        .character_size(serial::CharacterSize::EightBits)
        .mode(serial::Mode::Asynchronous)
        .parity(serial::Parity::Disabled)
        .stop_bits(serial::StopBits::OneBit)
        .configure();

    lcd::print("Ok!");
    //delay_ms(1500);
    lcd::clear();
    
    #[allow(arithmetic_overflow)]
    //1B 02 C1 50 61 02 1B 03 87 (valid posijet version1 master-to-slave cmpp frame)
    let frame: [u8;9] = [0x1B, 0x02, 0xC1-0xC1, 0x50+1, 0x61, 0x02, 0x1B, 0x03, 0x87+0xC1-1];
    
    for b in frame {
        serial::transmit(b);
    };

    let mut buf: [u8;12] = [0;12]; 
    let mut i=0;
    loop {
        if (serial::ready_to_receive()) {
            let b = serial::receive();
            buf[i] = b;
            i = i + 1;
            if (i>8) {
                for byte in buf {
                    lcd::print_u8_in_hex(byte);
                    lcd::print_char(';');
                }
                break;
            }
            
        }
    };

    lcd::print("#");
    
    loop {
    
    }

}