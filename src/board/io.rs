//high level io interface

use crate::microcontroler::delay::delay_ms;

use super::output_expander::{OutputExpander, OutputExpanderSignal, self};

use OutputExpanderSignal::*;

struct IO {
    output_expander: OutputExpander
}

impl IO {

    fn new() -> Self {
        let o = OutputExpander::new(); 
        IO {
            output_expander: o,
        }
    }

    fn commit(&self) -> () {
        self.output_expander.commit();
    }

    fn stage_signal(&mut self, msg: OutputExpanderSignal) -> &mut Self {
        self.output_expander.stage_signal(msg);
        self
    }

    fn buzzer(&mut self, on_off: bool) -> &mut Self {
        self.stage_signal(OutputExpanderSignal::BUZZER(on_off))
    }

    fn led_erro(&mut self, on_off: bool) -> &mut Self {
        self.stage_signal(OutputExpanderSignal::LED_ERRO(on_off))
    }

    fn led_execucao(&mut self, on_off: bool) -> &mut Self {
        self.stage_signal(OutputExpanderSignal::LED_EXECUCAO(on_off))
    }

    fn led_manual(&mut self, on_off: bool) -> &mut Self {
        self.stage_signal(OutputExpanderSignal::LED_MANUAL(on_off))
    }

    fn led_posicao_alcancada(&mut self, on_off: bool) -> &mut Self {
        self.stage_signal(OutputExpanderSignal::LED_POS_ALC(on_off))
    }

    fn led_programa(&mut self, on_off: bool) -> &mut Self {
        self.stage_signal(OutputExpanderSignal::LED_PROGRAMA(on_off))
    }

}



// ---------------------



pub fn development_entry_point() -> ! {

    let mut io = IO::new();

    loop {
        io
            .buzzer(true)
            .led_erro(true)
            .led_manual(true)
            .led_execucao(true)
            .led_posicao_alcancada(true)
            .led_programa(true)
            .commit();
        delay_ms(100);
        io
            .buzzer(false)
            .led_erro(false)
            .led_manual(false)
            .led_execucao(false)
            .led_posicao_alcancada(false)
            .led_programa(false)
            .commit();
        delay_ms(100);

        
    }
}