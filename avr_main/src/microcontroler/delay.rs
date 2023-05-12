//! Just a wrapper over the ruduino lib

use ruduino::delay::{delay_ms as delay_ms__, delay_us as delay_us__};

/// Blocking delay given miliseconds value
///
/// The ruduino delay appears to clamp miliseconds values above 1024.
/// Here we will allow values greater than that.
pub fn delay_ms(total_ms: u64) -> () {
    delay_ms__(total_ms);

    // below code not working correctly, fix when possible

    /*
    let size:u64 = 500; // arbitrary value, preferable less than 1024
    if total_ms > size {
        let secs = total_ms / size; // result has no reminder
        let reminder:u64 = total_ms - (secs*size);
        for _ in 0..secs {
            delay_ms__(size);
        }
        delay_ms__(reminder);
    } else {
        delay_ms__(size);
    }
    */
}

pub fn delay_us(total_us: u64) -> () {
    delay_us__(total_us)
}
