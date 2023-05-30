use crate::board::lcd::driver;

pub fn print_error_code_and_panics(error_code: u8) -> ! {
    // Assumes that lcd has already been initialized.
    driver::clear();
    driver::print("Err:");
    driver::print_u8_in_hex(error_code);
    // TODO: is this delay really necessary ?
    //delay_ms(4000);
    panic!()
}

/// TODO: Place this module inside the [`panic.rs`] module
#[macro_export]
macro_rules! fatal_error {
    ($error_code: literal) => {
        // TODO: Improve error handling when possible

        {
            use crate::fatal_error::print_error_code_and_panics;
            print_error_code_and_panics($error_code)
        }
    };
}
