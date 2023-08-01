use crate::board::front_panel::FrontPanel;
use crate::board::keyboard::Keyboard;
use crate::board::keypad::KeyCode;
use crate::board::lcd::driver::print_u8_in_hex;
use crate::board::peripherals::Peripherals;
use crate::board::peripherals::PeripheralsAvrHardware;
use crate::geometry::point::Point;
use crate::menu::model::DataModel;
use crate::menu::screen_buffer;
use crate::menu::screen_buffer::ScreenBuffer;
use crate::menu::widget::execucao::MenuExecucaoControler;
use crate::menu::widget::main_menu::MainMenu;
use crate::menu::widget::manual_mode::ManualModeMenuControler;
use crate::menu::widget::splash::Splash;
use crate::menu::widget::submenu_programa::menu_programa_controler::MenuProgramaControler;
use crate::menu::widget::submenu_programa::spec::{MenuProgramaAreanaSelector, MenuProgramaArena};
use crate::menu::widget::widget::Widget;
use crate::microcontroler::delay;
use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::timer::now;
use crate::microcontroler::{serial, timer};
use cross_platform::protocol::datalink::datalink::Datalink;
use cross_platform::protocol::transport::channel::Channel;
use cross_platform::protocol::transport::transport_layer::cmpp_value::MechanicalProperties;
use cross_platform::protocol::transport::transport_layer::TransportLayer;
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
    // Serial port
    serial::init(2400);
    let peripherals = PeripheralsAvrHardware::new();

    // lcd display buffer
    let mut screen_buffer = peripherals.get_screen_buffer();

    loop {
        screen_buffer.clear();
        let h = hmac_sha256::HMAC::mac(b"hello", b"key");
        print_hex(&mut screen_buffer, &h);
        screen_buffer.render();

        delay_ms(1000);

        screen_buffer.clear();
        let h = hmac_sha256::Hash::hash(b"hello");
        print_hex(&mut screen_buffer, &h);
        screen_buffer.render();

        delay_ms(1000);
    }
}
