use crate::{
    board::{lcd, output_expander::OutputExpander},
    enviroment::front_panel::FrontPanel,
    menu::{canvas::Canvas, keyboard::Keyboard},
    microcontroler::{serial, timer::init_timer},
};
use avr_progmem::progmem;

// ================= OPTIONAL WIDGET TEST ==========================================================

progmem! {
    static progmem string O1 = "Ligado";
    static progmem string O2 = "Deslig";
}

//static mut CURSOR: Cursor = Cursor::new(0..2, 0);
//
pub fn optional_widget_test() /*-> ! */
{
    //    let SystemEnviroment{mut canvas, mut keyboard, ..} = SystemEnviroment::new();
    //
    //    canvas.render();
    //
    //    //optional
    //
    //    let mut options = Vec::new();
    //    options.push(FlashString::new(&O1));
    //    options.push(FlashString::new(&O2));
    //    let accessor = Accessor::new(unsafe { &mut CURSOR });
    //
    //    let mut optional = Optional::new(options, accessor);
    //    let point = Point::new(0,0);
    //    let is_in_editing_mode = true;
    //    loop {
    //        if let Some(key) = keyboard.get_key() {
    //            optional.send_key(key);
    //        }
    //        optional.update();
    //        optional.draw(&mut canvas, point, is_in_editing_mode);
    //        canvas.render();
    //    }
    //
}
