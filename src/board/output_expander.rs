// high level abstraction over board ios

#![allow(non_camel_case_types)]

use super::shiftout::ShiftOutData;

use crate::{common::configure_bit, microcontroler::delay::delay_ms};

use super::shiftout::{init_shiftout_pins, write_shiftout};


enum OutputExpanderSignal {
    KBD_SA(bool),              // OUTPUT_BUS0     KBD-SA                   BIT0 - SHIFT-REGISTER 1 BEGIN
    KBD_SB(bool),              // OUTPUT_BUS1     KBD-SB                   BIT1             
    KBD_S1(bool),              // OUTPUT_BUS2     KBD-S1                   BIT2
    KBD_S2(bool),              // OUTPUT_BUS3     KBD-S2                   BIT3
    KBD_S3(bool),              // OUTPUT_BUS4     KBD-S3                   BIT4
    KBD_S4(bool),              // OUTPUT_BUS5     KBD-S4                   BIT5
    KBD_S5(bool),              // OUTPUT_BUS6     KBD-S5                   BIT6
    KBD_S6(bool),              // OUTPUT_BUS7     KBD-S6                   BIT7
    KBD_S7(bool),              // OUTPUT_BUS8     KBD-S7                   BIT0 - SHIFT-REGISTER 2 BEGIN
    KBD_S8(bool),              // OUTPUT_BUS9     KBD-S8                   BIT1
    SAIDA_VAGO1(bool),         // OUTPUT_BUS10    SAIDA-VAGO1              BIT2
    COPIA_SINAL_PTR(bool),     // OUTPUT_BUS11    COPIA-SINAL-PTR          BIT3
    SAIDA_START_OUTRO(bool),   // OUTPUT_BUS12    SAIDA-START-OUTRO        BIT4
    INVMEN(bool),              // OUTPUT_BUS13    INVMEN                   BIT5
    P3(bool),                  // OUTPUT_BUS14    P3                       BIT6
    P2(bool),                  // OUTPUT_BUS15    P2                       BIT7
    P1(bool),                  // OUTPUT_BUS16    P1                       BIT0 - SHIFT-REGISTER 3 BEGIN
    P0(bool),                  // OUTPUT_BUS17    P0                       BIT1
    DMOTOR_1(bool),            // OUTPUT_BUS18    DMOTOR-1                 BIT2
    DMOTOR_2(bool),            // OUTPUT_BUS19    DMOTOR-2                 BIT3
    EMOTOR_1(bool),            // OUTPUT_BUS20    EMOTOR-1                 BIT4
    EMOTOR_2(bool),            // OUTPUT_BUS21    EMOTOR-2                 BIT5
    NMOTOR_1(bool),            // OUTPUT_BUS22    NMOTOR-1                 BIT6
    NMOTOR_2(bool),            // OUTPUT_BUS23    NMOTOR-2                 BIT7
    HF_1(bool),                // OUTPUT_BUS24    H/F-1                    BIT0 - SHIFT-REGISTER 4 BEGIN
    HF_2(bool),                // OUTPUT_BUS25    H/F-2                    BIT1
    BUZZER(bool),              // OUTPUT_BUS26    BUZZER                   BIT2
    LED_POS_ALC(bool),         // OUTPUT_BUS27    LED_POS_ALC              BIT3
    LED_PROGRAMA(bool),        // OUTPUT_BUS28    LED_PROGRAMA             BIT4
    LED_ERRO(bool),            // OUTPUT_BUS29    LED_ERRO                 BIT5
    LED_EXECUCAO(bool),        // OUTPUT_BUS30    LED_EXECUCAO             BIT6
    LED_MANUAL(bool),          // OUTPUT_BUS31    LED_MANUAL               BIT7
}

enum ShiftRegister {
    Register0=0,
    Register1=1,
    Register2=2,
    Register3=3,
}

struct OutputExpander {
    stage_area: ShiftOutData,   
    has_changed: bool,          // avoid to send data to hardware if it doesn't has changed since last commit
}

impl OutputExpander {

  

    // call this function before all others
    fn new() -> Self {
        init_shiftout_pins();
        OutputExpander {
            stage_area: ShiftOutData {
                byte0: 0x00,
                byte1: 0x00,
                byte2: 0x00,
                byte3: 0x00,
            },
            has_changed: true,  // first commit we must send all
        }
    }
    
    // send all signais in the staged area to hardware
    fn commit(&self) -> () {
        // avoid to send data if nothing has changed
        if self.has_changed {
            write_shiftout(&self.stage_area);
        }
    }

    // helper function
    fn set_signal__(&mut self, register: u8, position: u8, data_bit: bool) -> () {
        //FIXME: Below code is unnecessary repetitive. Refactor it when possible!
        match register {
            0 => { 
                let oldByte = self.stage_area.byte0;
                let newByte = configure_bit(self.stage_area.byte0, position, data_bit);
                if newByte==oldByte {
                    self.has_changed = false;
                } else {
                    self.has_changed = true;
                    self.stage_area.byte0 = newByte;
                }
            },
            1 => { 
                let oldByte = self.stage_area.byte1;
                let newByte = configure_bit(self.stage_area.byte1, position, data_bit);
                if newByte==oldByte {
                    self.has_changed = false;
                } else {
                    self.has_changed = true;
                    self.stage_area.byte1 = newByte;
                }
            },
            2 => { 
                let oldByte = self.stage_area.byte2;
                let newByte = configure_bit(self.stage_area.byte2, position, data_bit);
                if newByte==oldByte {
                    self.has_changed = false;
                } else {
                    self.has_changed = true;
                    self.stage_area.byte2 = newByte;
                }
            },
            3 => { 
                let oldByte = self.stage_area.byte3;
                let newByte = configure_bit(self.stage_area.byte3, position, data_bit);
                if newByte==oldByte {
                    self.has_changed = false;
                } else {
                    self.has_changed = true;
                    self.stage_area.byte3 = newByte;
                }
            },
            _ => unreachable!("Error: trying to index an invalid shift register output"),
        };
    }

    // save signal to stage area but not send it to hardware
    fn stage_signal(&mut self, signal: OutputExpanderSignal) -> () {
        match signal {
            OutputExpanderSignal::KBD_SA(data) =>              self.set_signal__(0, 0, data),
            OutputExpanderSignal::KBD_SB(data) =>              self.set_signal__(0, 1, data),
            OutputExpanderSignal::KBD_S1(data) =>              self.set_signal__(0, 2, data),
            OutputExpanderSignal::KBD_S2(data) =>              self.set_signal__(0, 3, data),
            OutputExpanderSignal::KBD_S3(data) =>              self.set_signal__(0, 4, data),
            OutputExpanderSignal::KBD_S4(data) =>              self.set_signal__(0, 5, data),
            OutputExpanderSignal::KBD_S5(data) =>              self.set_signal__(0, 6, data),
            OutputExpanderSignal::KBD_S6(data) =>              self.set_signal__(0, 7, data),
            OutputExpanderSignal::KBD_S7(data) =>              self.set_signal__(1, 0, data),
            OutputExpanderSignal::KBD_S8(data) =>              self.set_signal__(1, 1, data),
            OutputExpanderSignal::SAIDA_VAGO1(data) =>         self.set_signal__(1, 2, data),
            OutputExpanderSignal::COPIA_SINAL_PTR(data) =>     self.set_signal__(1, 3, data),
            OutputExpanderSignal::SAIDA_START_OUTRO(data) =>   self.set_signal__(1, 4, data),
            OutputExpanderSignal::INVMEN(data) =>              self.set_signal__(1, 5, data),
            OutputExpanderSignal::P3(data) =>                  self.set_signal__(1, 6, data),
            OutputExpanderSignal::P2(data) =>                  self.set_signal__(1, 7, data),
            OutputExpanderSignal::P1(data) =>                  self.set_signal__(2, 0, data),
            OutputExpanderSignal::P0(data) =>                  self.set_signal__(2, 1, data),
            OutputExpanderSignal::DMOTOR_1(data) =>            self.set_signal__(2, 2, data),
            OutputExpanderSignal::DMOTOR_2(data) =>            self.set_signal__(2, 3, data),
            OutputExpanderSignal::EMOTOR_1(data) =>            self.set_signal__(2, 4, data),
            OutputExpanderSignal::EMOTOR_2(data) =>            self.set_signal__(2, 5, data),
            OutputExpanderSignal::NMOTOR_1(data) =>            self.set_signal__(2, 6, data),
            OutputExpanderSignal::NMOTOR_2(data) =>            self.set_signal__(2, 7, data),
            OutputExpanderSignal::HF_1(data) =>                self.set_signal__(3, 0, data),
            OutputExpanderSignal::HF_2(data) =>                self.set_signal__(3, 1, data),
            OutputExpanderSignal::BUZZER(data) =>              self.set_signal__(3, 2, data),
            OutputExpanderSignal::LED_POS_ALC(data) =>         self.set_signal__(3, 3, data),
            OutputExpanderSignal::LED_PROGRAMA(data) =>        self.set_signal__(3, 4, data),
            OutputExpanderSignal::LED_ERRO(data) =>            self.set_signal__(3, 5, data),
            OutputExpanderSignal::LED_EXECUCAO(data) =>        self.set_signal__(3, 6, data),
            OutputExpanderSignal::LED_MANUAL(data) =>          self.set_signal__(3, 7, data),
        }
    }


}


//

pub fn development_entry_point() -> ! {
    
    let mut output = OutputExpander::new();

    loop {
        output.stage_signal(OutputExpanderSignal::BUZZER(false));
        output.stage_signal(OutputExpanderSignal::LED_MANUAL(true));
        output.commit();
        delay_ms(50);
        output.stage_signal(OutputExpanderSignal::BUZZER(false));
        output.stage_signal(OutputExpanderSignal::LED_MANUAL(false));
        output.commit();
        delay_ms(50);
        
    }
    

}

 