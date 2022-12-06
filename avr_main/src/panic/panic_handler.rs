
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
    loop { }
}
