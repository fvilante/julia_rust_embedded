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
    front_panel::{FrontPanel, FrontPanelAvrHardware},
    input_expander::InputExpander,
    keyboard::{Keyboard, KeyboardAvrDriver},
    lcd::adapter::LcdHardware40x2,
    output_expander::OutputExpander,
};

/// Abstraction over the platform specific hardware instantiation
/// TODO: Move to a better place
pub trait Peripherals {
    fn get_keyboard<'a>(&'a self) -> KeyboardAvrDriver<'a>;
    fn get_front_panel<'a>(&'a self) -> FrontPanelAvrHardware<'a>;
    fn get_screen_buffer(&self) -> ScreenBuffer;
}

/// On board peripherals initialization
pub struct PeripheralsAvrHardware {
    output_expander: OutputExpander,
    input_expander: InputExpander,
    hardware_lcd: LcdHardware40x2,
}

impl PeripheralsAvrHardware {
    /// Initialize peripherals
    ///
    /// NOTE: Call this function once during the entire lifetime of the program
    pub fn new() -> impl Peripherals {
        ////////////////////////////////
        // Low Level initialization
        ////////////////////////////////

        // Initialize timer couting (1khz)
        init_timer();

        // Initialize on-board IO Expander
        Self {
            output_expander: OutputExpander::new(),
            input_expander: InputExpander::new(),
            hardware_lcd: LcdHardware40x2::new(),
        }
    }
}

impl Peripherals for PeripheralsAvrHardware {
    fn get_keyboard<'a>(&'a self) -> KeyboardAvrDriver<'a> {
        let keyboard = KeyboardAvrDriver::new(&self.output_expander, &self.input_expander);
        keyboard
    }

    fn get_front_panel<'a>(&'a self) -> FrontPanelAvrHardware<'a> {
        // Leds from the frontal panel
        let front_panel = FrontPanelAvrHardware::new(&self.output_expander);
        front_panel
    }

    fn get_screen_buffer(&self) -> ScreenBuffer {
        let lcd = &self.hardware_lcd;
        ScreenBuffer::new(lcd)
    }
}
