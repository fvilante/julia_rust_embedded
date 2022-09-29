// Wrapper over third-party hardware microcontroller's serial driver
use ruduino::legacy::serial;

use crate::{
    board::lcd, 
};

use super::delay::delay_ms;


/// you must call this function before call other serial related functions 
pub fn init(baud_rate: u32) {
    let BAUD: u32 = baud_rate;
    let UBRR: u16 = (ruduino::config::CPU_FREQUENCY_HZ / 16 / BAUD - 1) as u16;
    serial::Serial::new(UBRR)
        .character_size(serial::CharacterSize::EightBits)
        .mode(serial::Mode::Asynchronous)
        .parity(serial::Parity::Disabled)
        .stop_bits(serial::StopBits::OneBit)
        .configure();
}

// non-blocking checking if there exists a byte to be transmit
pub fn ready_to_transmit() -> bool {
    serial::ready_to_transmit()
}

/// Does a blocking transfer of one byte
pub fn transmit(byte: u8) {
    serial::transmit(byte)
}

// Non-blocking transmission of one byte and OK, or if not ready to transmit yet return Err 
pub fn try_transmit(byte: u8) -> Result<(), ()> {
    serial::try_transmit(byte)
}

// non-blocking checking if there exists a byte to be received
pub fn ready_to_receive() -> bool {
    serial::ready_to_receive()
}

/// Does a blocking read of one byte
pub fn receive() -> u8 {
    serial::receive()
}

/// non-blocking try to read one byte from serial, if `Some` byte available returns it, else returns `None`
pub fn try_receive() -> Option<u8> {
   serial::try_receive()
}



//

pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();
    lcd::print("Inicializado serial");

    init(9600);

    lcd::print("Ok!");
    //delay_ms(1500);
    lcd::clear();
    
    #[allow(arithmetic_overflow)]
    //1B 02 C1 50 61 02 1B 03 87 (valid posijet version1 master-to-slave cmpp frame)
    let frame: [u8;9] = [0x1B, 0x02, 0xC1-0xC1, 0x50+1, 0x61, 0x02, 0x1B, 0x03, 0x87+0xC1-1];
    
    for b in frame {
        transmit(b);
    };

    let mut buf: [u8;12] = [0;12]; 
    let mut i=0;
    loop {
        if ready_to_receive() {
            let b = receive();
            buf[i] = b;
            i = i + 1;
            if i>8 {
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