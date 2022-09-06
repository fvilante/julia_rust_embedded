// the ruduino delay appears to clamp miliseconds values above 1024.
// here we will allow values greater than that.

use ruduino::delay::{
    delay_ms as delay_ms__
};

pub fn delay_ms(total_ms: u64) -> () {
    let size:u64 = 500; // arbitrary value, preferable less than 1024
    if total_ms > size {
        let mut secs = total_ms / size; // result has no reminder
        let reminder:u64 = total_ms - (secs*size);
        for _ in 0..secs {
            delay_ms__(size);
        }
        delay_ms__(reminder);
    } else {
        delay_ms__(size);
    }
}

