use crate::board::lcd;
use alloc::alloc::Layout;

#[alloc_error_handler]
fn alloc_error_handler(_layout: Layout) -> ! {
    lcd::lcd_initialize();
    lcd::print("AllocErr");
    loop {}
}
