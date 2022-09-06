// Julia printed circuit board

use ruduino::Pin;
use ruduino::cores::atmega328p::{port};
use ruduino::delay::{delay_ms};

const LED_ON:bool = true;
const LED_OFF:bool = false;

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

pub fn blink_led3(on_interval_ms: u16, off_interval_ms: u16) -> ! {
    loop {
        set_led3(LED_ON);
        delay_ms(on_interval_ms.into());
        set_led3(LED_OFF);
        delay_ms(off_interval_ms.into());
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

