// A wrapper around the wold lcd.rs with the intention to improve client's api

use crate::board::lcd;

pub struct Lcd {}

impl Lcd {
    pub fn new() -> Self {
        lcd::lcd_initialize();
        Lcd {}
    }

    pub fn setCursor(&self, col: u8, row: u8) -> &Self {
        lcd::setCursor(col, row);
        self
    }

    pub fn print(&self, text: &str) -> &Self {
        lcd::print(text);
        self
    }

    pub fn print_u8_in_hex(&self, value: u8) -> &Self {
        lcd::print_u8_in_hex(value);
        self
    }

    pub fn print_char(&self, c: char) -> &Self {
        lcd::print_char(c);
        self
    }
}

// ------------------------------------------
// Examples

pub fn development_entry_point() -> ! {
    let lcd = Lcd::new();
    let mut i = 0;
    lcd.print("Contando").print(" de 0 ate -").print_char('>');

    lcd.print(" ");

    loop {
        lcd.setCursor(2, 1).print_u8_in_hex(i);
        i = i + 1;
    }
}
