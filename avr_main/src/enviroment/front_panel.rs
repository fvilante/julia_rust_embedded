use crate::{
    board::output_expander::{self, OutputExpander},
    microcontroler::delay::delay_ms,
};

pub struct FrontPanel<'a> {
    output_expander: &'a mut OutputExpander,
}

impl<'a> FrontPanel<'a> {
    pub fn new(output_expander: &'a mut OutputExpander) -> Self {
        Self { output_expander }
    }

    pub fn all(&mut self, on: bool) -> &mut Self {
        self.LED_ERRO(on);
        self.LED_POS_ALC(on);
        self.LED_MANUAL(on);
        self.LED_EXECUCAO(on);
        self.LED_PROGRAMA(on);
        self.BUZZER(on);
        self
    }

    /// blink fast all leds including buzzer
    pub fn auto_test(&mut self) -> &mut Self {
        self.all(false);
        self.all(true);
        delay_ms(200);
        self.all(false);
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.all(false);
        self
    }

    pub fn LED_ERRO(&mut self, on: bool) -> &mut Self {
        self.output_expander.LED_ERRO(on).commit();
        self
    }

    pub fn LED_POS_ALC(&mut self, on: bool) -> &mut Self {
        self.output_expander.LED_POS_ALC(on).commit();
        self
    }

    pub fn BUZZER(&mut self, on: bool) -> &mut Self {
        self.output_expander.BUZZER(on).commit();
        self
    }

    pub fn LED_MANUAL(&mut self, on: bool) -> &mut Self {
        self.output_expander.LED_MANUAL(on).commit();
        self
    }

    pub fn LED_EXECUCAO(&mut self, on: bool) -> &mut Self {
        self.output_expander.LED_EXECUCAO(on).commit();
        self
    }

    pub fn LED_PROGRAMA(&mut self, on: bool) -> &mut Self {
        self.output_expander.LED_PROGRAMA(on).commit();
        self
    }

    //

    pub fn Beep(&mut self) -> &mut Self {
        self.output_expander.BUZZER(true).commit();
        delay_ms(50);
        self.output_expander.BUZZER(false).commit();
        self
    }
}

pub fn development_entry_point() -> ! {
    let mut output_expander = OutputExpander::new();

    FrontPanel::new(&mut output_expander).reset().auto_test();

    loop {}
}
