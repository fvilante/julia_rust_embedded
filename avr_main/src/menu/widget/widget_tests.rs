use crate::{
    board::{lcd, output_expander::OutputExpander},
    enviroment::front_panel::FrontPanel,
    menu::{canvas::Canvas, keyboard::Keyboard},
    microcontroler::{serial, timer::init_timer},
};
use avr_progmem::progmem;

pub struct SystemEnviroment {
    pub output_expander: OutputExpander,
    pub keyboard: Keyboard,
    pub canvas: Canvas,
}

impl SystemEnviroment {
    /// Initialize system peripherals:
    /// * timer interruption (at 1khz)
    /// * serial port
    /// * lcd display
    /// * keyboard
    /// * general I/O and canvas
    /// * front panel leds
    pub fn new(baud_rate: u32) -> Self {
        // initialize timer couting (1khz)
        init_timer();
        // serial port
        serial::init(baud_rate);
        // lcd display
        lcd::lcd_initialize();
        // general I/O
        let output_expander = OutputExpander::new();
        // keyboard
        let beep = |on: bool| {
            OutputExpander::new().BUZZER(on).commit();
        };
        let keyboard = Keyboard::new(beep);
        // canvas
        let canvas = Canvas::new();
        Self {
            output_expander,
            keyboard,
            canvas,
        }
    }

    /// Give access to front panel leds
    pub fn get_front_panel<'a>(&'a mut self) -> FrontPanel<'a> {
        let front_panel: FrontPanel<'a> = FrontPanel::new(&mut self.output_expander);
        front_panel
    }
}

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
