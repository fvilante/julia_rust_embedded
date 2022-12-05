
use core::panic::PanicInfo;
use alloc::string::ToString;

use crate::board::lcd;


// PANIC HANDLER {{{
// =============
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {

    //TODO: Disable interrupts (?!) Not for now because there are no much interruptions running under the hood
    
    lcd::lcd_initialize();
    lcd::clear();

    //TODO: I didn't explored the `info` object and do not know yet how to take the `.expect` message
    if let Some(s) = info.message() {
        let s = *s;
        lcd::print("ERROR: ");
        lcd::print(s.as_str().unwrap());
    } else {
        lcd::print("FATAL ERROR: NO MESSAGE");
    }


    loop {
        // HALT
    }

}