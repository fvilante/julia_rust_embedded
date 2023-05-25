//! This module is responsible for on-board peripherals initialization:
//!
//!   * Timer interruption (at 1khz)
//!   * Lcd display
//!   * Input & output ports expander
//!   * Front panel
//!   * Keyboard
//!   * Canvas
//!
//! NOTE: We are initializing serial port in other place, not here.
use crate::{menu::screen_buffer::ScreenBuffer, microcontroler::timer::init_timer};

use super::{
    front_panel::FrontPanel,
    input_expander::InputExpander,
    keyboard::{Keyboard, KeyboardAvrDriver},
    lcd::{self, adapter::LcdHardware40x2, interface::Lcd},
    output_expander::OutputExpander,
};

/// On board peripherals initialization
pub struct Peripherals {
    output_expander: OutputExpander,
    input_expander: InputExpander,
    hardware_lcd: LcdHardware40x2,
}

impl Peripherals {
    /// Initialize peripherals
    ///
    /// NOTE: Call this function once during the entire lifetime of the program
    pub fn new() -> Self {
        ////////////////////////////////
        // Low Level initialization
        ////////////////////////////////

        // Initialize timer couting (1khz)
        init_timer();

        // Lcd display
        lcd::lcd_initialize();
        // Initialize on-board IO Expander
        Self {
            output_expander: OutputExpander::new(),
            input_expander: InputExpander::new(),
            hardware_lcd: LcdHardware40x2::new(),
        }
    }

    pub fn get_keyboard(&self) -> impl Keyboard + '_ {
        let keyboard = KeyboardAvrDriver::new(&self.output_expander, &self.input_expander);
        keyboard
    }

    pub fn get_front_panel(&self) -> FrontPanel {
        // Leds from the frontal panel
        let front_panel = FrontPanel::new(&self.output_expander);
        front_panel
    }

    pub fn get_screen_buffer(&self) -> ScreenBuffer {
        let lcd = &self.hardware_lcd;
        ScreenBuffer::new(lcd)
    }
}
