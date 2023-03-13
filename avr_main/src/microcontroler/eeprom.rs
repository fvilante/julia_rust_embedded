use ruduino::prelude::without_interrupts;

use crate::{
    board::lcd,
    menu::canvas::Canvas,
    microcontroler::{
        delay::delay_ms,
        register::{read_register, write_register},
    },
};

use lib_1::{
    protocol::datalink::datalink::word16::Word16,
    utils::common::{get_bit_at, word_to_byte},
};

//EEPROM registers addresses and bits
const EEARH: *mut u8 = 0x42 as *mut u8;
const EEARL: *mut u8 = 0x41 as *mut u8;
const EECR: *mut u8 = 0x3F as *mut u8;
const EEDR: *mut u8 = 0x40 as *mut u8;
const EEPE: u8 = 0b0000010;
const EEMPE: u8 = 0b0000100;
const EERE: u8 = 0b0000001;

/// normalize is to clamp the 16 bits address to a 9 bits address (1+8).
/// only bit 9 of address16 are used, other bits are reserved by the microcontroller hardware.
fn normalize_eeprom_address(address: u16) -> (u8, u8) {
    let (byte_low, byte_high) = Word16::from_u16(address).split_bytes();
    let address_high = get_bit_at(byte_high, 0);
    let address_low = byte_low;
    (address_high, address_low)
}

pub fn read_eeprom(address: u16) -> u8 {
    let (address_high, address_low) = normalize_eeprom_address(address);
    // set EEPROM address register
    write_register(EEARH, address_high);
    write_register(EEARL, address_low);
    //do read
    while (read_register(EECR) & (1 << EEPE)) == 1 {} // wait until EEPE become to zero by hardware
    write_register(EECR, read_register(EECR) | (1 << EERE));
    read_register(EEDR)
}

pub fn write_eeprom(address: u16, data: u8) -> () {
    // set EEPROM address and data register
    let (address_high, address_low) = normalize_eeprom_address(address);
    write_register(EEARH, address_high);
    write_register(EEARL, address_low);
    write_register(EEDR, data);
    // write operation
    while (read_register(EECR) & (1 << EEPE)) == 1 {} // wait until EEPE become to zero
    without_interrupts(|| {
        write_register(EECR, read_register(EECR) | (1 << EEMPE));
        write_register(EECR, read_register(EECR) | (1 << EEPE));
    });
}

pub struct EepromTestError {
    address: u16,
    expected_value: u8,
    actual_value: u8,
}

pub fn auto_test_eeprom(canvas: &mut Canvas) -> Result<(), EepromTestError> {
    use core::ops::Range;

    fn write_data_into_eeprom(range: Range<u8>) {
        for address in range {
            write_eeprom(address as u16, address);
        }
    }

    fn check_data_into_eeprom(range: Range<u8>) -> Result<(), EepromTestError> {
        for address in range {
            let data_read = read_eeprom(address as u16);
            if data_read == address {
                continue;
            } else {
                lcd::clear();
                lcd::print("address:");
                lcd::print_u8_in_hex(address);
                lcd::print(" ");
                lcd::print("expected_value:");
                lcd::print_u8_in_hex(address);
                lcd::print(" ");
                lcd::print("actual_value:");
                lcd::print_u8_in_hex(data_read);
                lcd::print(" ");
                delay_ms(5000);

                return Err(EepromTestError {
                    address: address as u16,
                    expected_value: address,
                    actual_value: data_read,
                });
            }
        }

        Ok(())
    }

    let range = 0x32..0x99;
    write_data_into_eeprom(range.clone());

    check_data_into_eeprom(range.clone())
}
