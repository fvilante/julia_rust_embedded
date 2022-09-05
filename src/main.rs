#![no_std]
#![no_main]

extern crate avr_std_stub;


use ruduino::Pin;
use ruduino::cores::current::{port};
use ruduino::prelude::without_interrupts;

/// The data direction register for PORT B, which is mapped to 0x24 in memory on the atmega328.
const DDRB: *mut u8 = 0x24 as *mut u8;
/// The pin status register for PORT B, which is mapped to 0x25 in memory on the atmega328.
const PORTB: *mut u8 = 0x25 as *mut u8;


//EEPROM
const EEARH: *mut u8 = 0x42 as *mut u8;
const EEARL: *mut u8 = 0x41 as *mut u8;
const EECR: *mut u8 = 0x3F as *mut u8;
const EEDR: *mut u8 = 0x40 as *mut u8;
const EEPE: u8 =  0b0000010;
const EEMPE: u8 = 0b0000100;
const EERE: u8 =  0b0000001;



#[no_mangle]
pub extern fn main() {
    port::B5::set_output();
    unsafe {

        let address: u8 = 0x10; // EPPROM address 
        let dataToWrite: u8 = 0x77+1; // data to write in EEPROM

        // set EEPROM address and data register
        core::ptr::write_volatile(EEARH, 0x00);
        core::ptr::write_volatile(EEARL, address);
        core::ptr::write_volatile(EEDR, dataToWrite);

        // write operation
        while (core::ptr::read_volatile(EECR) & (1<<EEPE)) == 1 { }; // wait until EEPE become to zero
        without_interrupts(|| {
            core::ptr::write_volatile(EECR, core::ptr::read_volatile(EECR) | (1<<EEMPE));
            core::ptr::write_volatile(EECR, core::ptr::read_volatile(EECR) | (1<<EEPE));
        });


        // read operation
        while (core::ptr::read_volatile(EECR) & (1<<EEPE)) == 1 { }; // wait until EEPE become to zero
        core::ptr::write_volatile(EECR, core::ptr::read_volatile(EECR) | (1<<EERE));
        let dataRead = core::ptr::read_volatile(EEDR);

        // if ok blink led fast, otherwise slow
        if(dataToWrite==dataRead) {
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
