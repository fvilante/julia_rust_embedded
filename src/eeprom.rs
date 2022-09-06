use ruduino::prelude::without_interrupts;

use crate::register:: {
    write_register,
    read_register,
};

use crate::common:: {
    word_to_byte,
    get_bit_at,
};

use crate::board:: {
    blink_led3,
};


//EEPROM registers addresses and bits
const EEARH: *mut u8 = 0x42 as *mut u8;
const EEARL: *mut u8 = 0x41 as *mut u8;
const EECR: *mut u8 = 0x3F as *mut u8;
const EEDR: *mut u8 = 0x40 as *mut u8;
const EEPE: u8 =  0b0000010;
const EEMPE: u8 = 0b0000100;
const EERE: u8 =  0b0000001;

// normalize is to clamp the 16 bits address to a 9 bits address (1+8)
fn normalize_eeprom_address(address: u16) -> (u8, u8) {
    let (address_low, address_high) = word_to_byte(address);
    let bit9 = get_bit_at(address_high, 0); // only bit 9 of address16 are used, other bits are reserved by the microcontroller hardware
    (bit9, address_low)
}

pub fn read_eeprom(address: u16) -> u8 {
    let (bit9, address_low) = normalize_eeprom_address(address);
    // set EEPROM address register
    write_register(EEARH, bit9);
    write_register(EEARL, address_low); 
    //do read
    while (read_register(EECR) & (1<<EEPE)) == 1 { }; // wait until EEPE become to zero by hardware
    write_register(EECR, read_register(EECR) | (1<<EERE));
    read_register(EEDR)
}

pub fn write_eeprom(address: u16, data: u8) -> () {
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




// ATTENTION: This routine erases EEPROM
pub fn hard_test_eeprom() -> ! {
    fn blink_led_fast() -> ! { blink_led3(100, 100) }
    fn blink_led_slow() -> ! { blink_led3(1000, 1000) }
    if test_eeprom() {
        blink_led_fast()
    } else {
        blink_led_slow()
    }
}




