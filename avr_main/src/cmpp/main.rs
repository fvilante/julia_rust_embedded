use crate::board::lcd;

pub fn development_entry_point() -> ! {
    lcd::lcd_initialize();
    lcd::print("Juca kifuri");

    loop {}
}
