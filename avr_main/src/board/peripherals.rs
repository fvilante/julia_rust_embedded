use crate::{
    enviroment::front_panel::FrontPanel,
    menu::{canvas::Canvas, keyboard::Keyboard},
    microcontroler::{serial, timer::init_timer},
};

use super::{input_expander::InputExpander, lcd, output_expander::OutputExpander};

/// On board peripherals initialization:
///
///   * Timer interruption (at 1khz)
///   * Serial port
///   * Lcd display
///   * Input & output ports expander
///   * Front panel
///   * Keyboard
///   * Canvas
pub struct Peripherals {
    output_expander: OutputExpander,
    input_expander: InputExpander,
}

impl<'a> Peripherals {
    /// Initialize peripherals
    ///
    /// NOTE: Call this function once during the entire lifetime of the program
    pub fn new(baud_rate: u32) -> Self {
        ////////////////////////////////
        // Low Level initialization
        ////////////////////////////////

        // Initialize timer couting (1khz)
        init_timer();
        // Serial port
        serial::init(baud_rate);
        // Lcd display
        lcd::lcd_initialize();
        // Initialize on-board IO Expander
        Self {
            output_expander: OutputExpander::new(),
            input_expander: InputExpander::new(),
        }
    }

    pub fn get_keyboard(&'a self) -> Keyboard<'a> {
        let keyboard = Keyboard::new(&self.output_expander, &self.input_expander);
        keyboard
    }

    pub fn get_front_panel(&'a self) -> FrontPanel<'a> {
        // Leds from the frontal panel
        let front_panel = FrontPanel::new(&self.output_expander);
        front_panel
    }

    pub fn get_canvas(&self) -> Canvas {
        let canvas = Canvas::new();
        canvas
    }
}
