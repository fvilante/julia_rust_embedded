#![no_std]
#![no_main]

extern crate avr_std_stub;

mod core;
mod register;
mod eeprom;

use ruduino::Pin;
use ruduino::cores::current::{port};

use ruduino::delay::delay_ms;
use eeprom:: {
    write_eeprom,
    read_eeprom,
    test_eeprom,
};

/// The data direction register for PORT B, which is mapped to 0x24 in memory on the atmega328.
const DDRB: *mut u8 = 0x24 as *mut u8;
/// The pin status register for PORT B, which is mapped to 0x25 in memory on the atmega328.
const PORTB: *mut u8 = 0x25 as *mut u8;



#[no_mangle]
pub extern fn main() {
    port::B5::set_output();
    // if ok blink led fast, otherwise slow
    if test_eeprom() {
        loop {
            port::B5::set_high();
            ruduino::delay::delay_ms(100);          
            port::B5::set_low();
            ruduino::delay::delay_ms(100);     
        }
    } else {
        loop {
            port::B5::set_high();
            ruduino::delay::delay_ms(1000);
            port::B5::set_low();
            ruduino::delay::delay_ms(1000);
        }
    }
}




/*#![no_std]
#![no_main]

use ruduino::Pin;
use ruduino::cores::current::{port};

#[no_mangle]
pub extern fn main() {
    port::B5::set_output();

    loop {
        port::B5::set_high();

        ruduino::delay::delay_ms(1000);

        port::B5::set_low();

        ruduino::delay::delay_ms(1000);
    }
}
*/

/*

        // Set the upper four physical pins on PORT B to inputs, the lower four to outputs.
        // The AVR interprets '1' in the data direction register as 'output', '0' input
        // for the corresponding pin.
        core::ptr::write_volatile(DDRB, core::ptr::read_volatile(DDRB) | 0b00100000);

        loop {
            // Write half of the output pins as high, the other half low.
            core::ptr::write_volatile(PORTB, 0b11111111);
            ruduino::delay::delay_ms(1000);
            core::ptr::write_volatile(PORTB, 0b00000000);
            ruduino::delay::delay_ms(1000);
        }
*/
