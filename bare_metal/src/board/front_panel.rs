//! Controls the leds and the buzzer of the front panel
//!
//! This is a high-level API over some outputs of the `output shift-register` driver.
//! These leds and the buzzer is parte of the interface human-machine and has
//! purpose to inform the user some events, like errors and keypressed.
//!
//! # Example
//!
//! ```
//! pub fn development_entry_point() -> ! {
//!     let mut output_expander = OutputExpander::new();
//!
//!     let front_panel = FrontPanelAvrHardware::new(&mut output_expander)
//!         
//!     front_panel.reset();
//!     front_panel.auto_test();
//!
//!     loop {}
//! }
//! ```
//!

/// Front panel controler abstraction. Controls panel leds and buzzer.
/// TODO: Move to a better place
pub trait FrontPanel {
    // required methods
    fn LED_ERRO(&mut self, on: bool);
    fn LED_POS_ALC(&mut self, on: bool);
    fn BUZZER(&mut self, on: bool);
    fn LED_MANUAL(&mut self, on: bool);
    fn LED_EXECUCAO(&mut self, on: bool);
    fn LED_PROGRAMA(&mut self, on: bool);

    // optinal methods

    /// Beeps for a particular duration and stops
    fn Beep(&mut self, duration_milisecs: u16) {
        self.BUZZER(true);
        delay_ms(duration_milisecs as u64);
        self.BUZZER(false)
    }

    fn all(&mut self, on: bool) {
        self.LED_ERRO(on);
        self.LED_POS_ALC(on);
        self.LED_MANUAL(on);
        self.LED_EXECUCAO(on);
        self.LED_PROGRAMA(on);
        self.BUZZER(on);
    }

    /// blink fast all leds including buzzer
    fn auto_test(&mut self) {
        self.all(false);
        self.all(true);
        delay_ms(200);
        self.all(false);
    }

    fn reset(&mut self) {
        self.all(false);
    }
}

//

use crate::{board::output_expander::OutputExpander, microcontroler::delay::delay_ms};

pub struct FrontPanelAvrHardware<'a> {
    output_expander: &'a OutputExpander,
}

impl<'a> FrontPanelAvrHardware<'a> {
    pub fn new(output_expander: &'a OutputExpander) -> Self {
        Self { output_expander }
    }
}

impl<'a> FrontPanel for FrontPanelAvrHardware<'a> {
    fn LED_ERRO(&mut self, on: bool) {
        self.output_expander.LED_ERRO(on).commit()
    }

    fn LED_POS_ALC(&mut self, on: bool) {
        self.output_expander.LED_POS_ALC(on).commit()
    }

    fn BUZZER(&mut self, on: bool) {
        self.output_expander.BUZZER(on).commit()
    }

    fn LED_MANUAL(&mut self, on: bool) {
        self.output_expander.LED_MANUAL(on).commit()
    }

    fn LED_EXECUCAO(&mut self, on: bool) {
        self.output_expander.LED_EXECUCAO(on).commit()
    }

    fn LED_PROGRAMA(&mut self, on: bool) {
        self.output_expander.LED_PROGRAMA(on).commit()
    }
}
