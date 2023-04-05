use arduino_hal::delay_ms;
use core::panic::PanicInfo;

use crate::board::lcd;

// PANIC HANDLER {{{
// =============
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    /// TODO: I noticed that using `PanicInfo::message` consumes a lot of memory (about 15% of total Flash, and 40% of total Ram)
    /// So I'm disable it here and given just a simple implamentation of error. The solution should be to use a global
    /// string to containg the error message, and this global string is setted before panic!() is colled, then here in
    /// this panic handler we can present this string.
    /// Code below exists just for reference, when the solution was implemented please delete it.
    /*
    let NO_MESSAGE1 = "NO_MESSAGE_1";
    let NO_MESSAGE2 = "NO_MESSAGE_2";

    let error_message = if let Some(arguments) = info.message() {
        if let Some(error_message) = arguments.as_str() {
            error_message
        } else {
            NO_MESSAGE2
        }
    } else {
        NO_MESSAGE1
    };
    lcd::print("ERROR: ");
    lcd::print(error_message);
    delay_ms(2000); // Just a delay for the case of recurssive calls to `panic` (for example, inside panic function, try to unwrap a None value).
    */
    lcd::lcd_initialize();
    lcd::clear();
    lcd::print("Err");

    loop {}
}

#[macro_export]
macro_rules! expect_result {
    ($v:expr, $msg:expr) => {
        match $v {
            Ok(v) => v,
            Err(_) => panic!($msg),
        }
    };
}

#[macro_export]
macro_rules! unwrap_option {
    ($v:expr, $msg:expr) => {
        match $v {
            Some(v) => v,
            None => panic!($msg),
        }
    };
}
