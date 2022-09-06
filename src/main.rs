#![no_std]
#![no_main]

extern crate avr_std_stub;


use ruduino::Pin;
use ruduino::cores::current::{port};
use ruduino::prelude::without_interrupts;
use ruduino::delay::delay_ms;

/// The data direction register for PORT B, which is mapped to 0x24 in memory on the atmega328.
const DDRB: *mut u8 = 0x24 as *mut u8;
/// The pin status register for PORT B, which is mapped to 0x25 in memory on the atmega328.
const PORTB: *mut u8 = 0x25 as *mut u8;


//EEPROM registers addresses and bits
const EEARH: *mut u8 = 0x42 as *mut u8;
const EEARL: *mut u8 = 0x41 as *mut u8;
const EECR: *mut u8 = 0x3F as *mut u8;
const EEDR: *mut u8 = 0x40 as *mut u8;
const EEPE: u8 =  0b0000010;
const EEMPE: u8 = 0b0000100;
const EERE: u8 =  0b0000001;


fn write_register(reg_address: *mut u8, value: u8) -> () {
    unsafe {
        core::ptr::write_volatile(reg_address, value);
    }
}

fn read_register(reg_address: *const u8) -> u8 {
    unsafe {
        core::ptr::read_volatile(reg_address)
    }
}

// return (byteLow, byteHigh)
fn word_to_byte(word: u16) -> (u8, u8) {
    let low_ = word & 0x00FF;
    let byte_low  = low_ as u8;
    let high_ = (word >> 8) & 0x00FF;
    let byte_high = high_ as u8;
    (byte_low, byte_high)
}

// position 0 = bit0, position 1 = bit1, ...
fn get_bit_at(data: u8, position: u8) -> u8 {
    data & (1 << 0)
}

// normalize is to clamp the 16 bits address to a 9 bits address (1+8)
fn normalize_eeprom_address(address: u16) -> (u8, u8) {
    let (address_low, address_high) = word_to_byte(address);
    let bit9 = get_bit_at(address_high, 0); // only bit 9 of address16 are used, other bits are reserved by the microcontroller hardware
    (bit9, address_low)
}

fn read_eeprom(address: u16) -> u8 {
    let (bit9, address_low) = normalize_eeprom_address(address);
    // set EEPROM address register
    write_register(EEARH, bit9);
    write_register(EEARL, address_low); 
    //do read
    while (read_register(EECR) & (1<<EEPE)) == 1 { }; // wait until EEPE become to zero by hardware
    write_register(EECR, read_register(EECR) | (1<<EERE));
    read_register(EEDR)
}

fn write_eeprom(address: u16, data: u8) -> () {
    // set EEPROM address and data register
    let (bit9, address_low) = normalize_eeprom_address(address);
    write_register(EEARH, bit9);
    write_register(EEARL, address_low);
    write_register(EEDR, data); 
    // write operation
    while (read_register(EECR) & (1<<EEPE)) == 1 { }; // wait until EEPE become to zero
    without_interrupts(|| {
        write_register(EECR, read_register(EECR) | (1<<EEMPE));
        write_register(EECR, read_register(EECR) | (1<<EEPE));
    });
}

// ATTENTION: This routine erases EEPROM
// write and read all eeprom addresses twice, return true if success
fn test_eeprom() -> bool {
    let data = 0x77;
    let mut result: bool = true;
    let eeprom_size: u16 = 1024; //1K bytes
    for address in 0..eeprom_size {
        write_eeprom(address, data);
        let data_read = read_eeprom(address);
        if data_read != data {
            result = false;
            break;
        }
    }
    result
}

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
