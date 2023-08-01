use crate::board::peripherals::Peripherals;
use crate::board::peripherals::PeripheralsAvrHardware;
use crate::menu::screen_buffer::ScreenBuffer;
use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::serial;
use cross_platform::utils::numerical::convert_u8_to_str_hex;

fn print_hex(lcd_: &mut ScreenBuffer, data: &[u8]) {
    for byte in data.into_iter() {
        let (high, low) = convert_u8_to_str_hex(byte.clone());
        lcd_.print_char(high);
        lcd_.print_char(low);
    }
}

/// Entry point of the main application
pub fn run() -> ! {
    let peripherals = PeripheralsAvrHardware::new();

    // lcd display buffer
    let mut lcd = peripherals.get_screen_buffer();

    loop {
        lcd.clear();
        let h = hmac_sha256::HMAC::mac(b"hello", b"key");
        print_hex(&mut lcd, &h);
        lcd.render();

        delay_ms(1000);

        lcd.clear();
        let h = hmac_sha256::Hash::hash(b"hello");
        print_hex(&mut lcd, &h);
        lcd.render();

        delay_ms(1000);
    }
}
