use arduino_hal::delay_ms;
use core::panic::PanicInfo;

use crate::board::lcd;

// PANIC HANDLER {{{
// =============
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lcd::lcd_initialize();
    lcd::clear();

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
