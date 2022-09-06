// Julia printed circuit board

use ruduino::Pin;
use ruduino::cores::atmega328p::{port};
use ruduino::delay::{delay_ms};

const LED_ON:bool = true;
const LED_OFF:bool = false;

fn set_led3(state: bool) -> () {
    port::B5::set_output();
    if state == LED_ON {
        port::B5::set_high()
    } else {
        port::B5::set_low()
    }
}

pub fn blink_led3(on_interval_ms: u16, off_interval_ms: u16) -> () {
    loop {
        set_led3(LED_ON);
        delay_ms(on_interval_ms.into());
        set_led3(LED_OFF);
        delay_ms(off_interval_ms.into());
    }
}

