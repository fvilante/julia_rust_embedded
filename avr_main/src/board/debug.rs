// Julia printed circuit board

use ruduino::cores::atmega328p::port;
use ruduino::Pin;

use crate::microcontroler::{delay::delay_ms, eeprom::read_eeprom, eeprom::write_eeprom};

// -------------------------------------------------------------------------------------------------------

const LED_ON: bool = true;
const LED_OFF: bool = false;

//static mut led3_state: bool = true; //current led state

pub fn set_led3(state: bool) -> () {
    port::B5::set_output();
    if state == LED_ON {
        //unsafe{ led3_state = true };
        port::B5::set_high()
    } else {
        //unsafe{ led3_state = false };
        port::B5::set_low()
    }
}

pub fn blink_led3(on_interval_ms: u64, off_interval_ms: u64, repeat: Option<u64>) -> () {
    let blink_once = |on_interval_ms: u64, off_interval_ms: u64| {
        set_led3(LED_ON);
        delay_ms(on_interval_ms);
        set_led3(LED_OFF);
        delay_ms(off_interval_ms);
    };

    match repeat {
        Some(n) => {
            for _ in 0..n {
                blink_once(on_interval_ms, off_interval_ms);
            }
        }
        None => {
            loop {
                blink_once(on_interval_ms, off_interval_ms);
            }
        }
    }
}

//FUNCTION NOT WORKING! PROBLEM A PROBLEM IN THE ITS GLOBAL STATE
/*
pub fn toogle_led3() -> () {
    let mut new_value:bool;
    unsafe{
        led3_state = !led3_state;
        new_value = led3_state;
    };
    set_led3(new_value)
}
*/

// -------------------------------------------------------------------------------------------------------

/*
    EEPROM TEST
    ATTENTION: Those routines erases EEPROM data
*/

// ATTENTION: This routine erases EEPROM data
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

// ATTENTION: This routine erases EEPROM data
pub fn hard_test_eeprom() -> () {
    fn blink_led_fast() -> () {
        blink_led3(100, 100, None)
    }
    fn blink_led_slow() -> () {
        blink_led3(1000, 1000, None)
    }
    if test_eeprom() {
        blink_led_fast()
    } else {
        blink_led_slow()
    }
}

// -------------------------------------------------------------------------------------------------------
