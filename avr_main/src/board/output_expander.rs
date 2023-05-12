// high level abstraction over board ios

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use super::shiftout::write_shiftout;
use super::shiftout::{init_shiftout_pins, ShiftOutData};
use crate::microcontroler::delay::delay_ms;
use core::cell::Cell;
use lib_1::utils::bit_wise::{configure_bit, get_bit_at_as_bool};

// See board schematic. This represents the electrical signals on the board
pub enum OutputExpanderSignal {
    KBD_SA,            // OUTPUT_BUS0     KBD-SA                   BIT0 - SHIFT-REGISTER 0 BEGIN
    KBD_SB,            // OUTPUT_BUS1     KBD-SB                   BIT1
    KBD_S1,            // OUTPUT_BUS2     KBD-S1                   BIT2
    KBD_S2,            // OUTPUT_BUS3     KBD-S2                   BIT3
    KBD_S3,            // OUTPUT_BUS4     KBD-S3                   BIT4
    KBD_S4,            // OUTPUT_BUS5     KBD-S4                   BIT5
    KBD_S5,            // OUTPUT_BUS6     KBD-S5                   BIT6
    KBD_S6,            // OUTPUT_BUS7     KBD-S6                   BIT7
    KBD_S7,            // OUTPUT_BUS8     KBD-S7                   BIT0 - SHIFT-REGISTER 1 BEGIN
    KBD_S8,            // OUTPUT_BUS9     KBD-S8                   BIT1
    SAIDA_VAGO1,       // OUTPUT_BUS10    SAIDA-VAGO1              BIT2
    COPIA_SINAL_PTR,   // OUTPUT_BUS11    COPIA-SINAL-PTR          BIT3
    SAIDA_START_OUTRO, // OUTPUT_BUS12    SAIDA-START-OUTRO        BIT4
    INVMEN,            // OUTPUT_BUS13    INVMEN                   BIT5
    P3,                // OUTPUT_BUS14    P3                       BIT6
    P2,                // OUTPUT_BUS15    P2                       BIT7
    P1,                // OUTPUT_BUS16    P1                       BIT0 - SHIFT-REGISTER 2 BEGIN
    P0,                // OUTPUT_BUS17    P0                       BIT1
    DMOTOR_1,          // OUTPUT_BUS18    DMOTOR-1                 BIT2
    DMOTOR_2,          // OUTPUT_BUS19    DMOTOR-2                 BIT3
    EMOTOR_1,          // OUTPUT_BUS20    EMOTOR-1                 BIT4
    EMOTOR_2,          // OUTPUT_BUS21    EMOTOR-2                 BIT5
    NMOTOR_1,          // OUTPUT_BUS22    NMOTOR-1                 BIT6
    NMOTOR_2,          // OUTPUT_BUS23    NMOTOR-2                 BIT7
    HF_1,              // OUTPUT_BUS24    H/F-1                    BIT0 - SHIFT-REGISTER 3 BEGIN
    HF_2,              // OUTPUT_BUS25    H/F-2                    BIT1
    BUZZER,            // OUTPUT_BUS26    BUZZER                   BIT2
    LED_POS_ALC,       // OUTPUT_BUS27    LED_POS_ALC              BIT3
    LED_PROGRAMA,      // OUTPUT_BUS28    LED_PROGRAMA             BIT4
    LED_ERRO,          // OUTPUT_BUS29    LED_ERRO                 BIT5
    LED_EXECUCAO,      // OUTPUT_BUS30    LED_EXECUCAO             BIT6
    LED_MANUAL,        // OUTPUT_BUS31    LED_MANUAL               BIT7
}

/// Represents each of the four 74HC595N Integrated Circuit present on the Julia board
#[derive(Copy, Clone)]
enum ShiftRegister {
    IC0, // Board descriptor: U110
    IC1, // Board descriptor: U101
    IC2, // Board descriptor: U104
    IC3, // Board descriptor: U103
}

enum Bit {
    D0 = 0, // bit 0 of a byte
    D1 = 1, // bit 1 of a byte
    D2 = 2, // etc...
    D3 = 3,
    D4 = 4,
    D5 = 5,
    D6 = 6,
    D7 = 7,
}

struct Address(ShiftRegister, Bit);

// Signals setup
fn getAddress__(signal: OutputExpanderSignal) -> Address {
    match signal {
        OutputExpanderSignal::KBD_SA => Address(ShiftRegister::IC0, Bit::D0),
        OutputExpanderSignal::KBD_SB => Address(ShiftRegister::IC0, Bit::D1),
        OutputExpanderSignal::KBD_S1 => Address(ShiftRegister::IC0, Bit::D2),
        OutputExpanderSignal::KBD_S2 => Address(ShiftRegister::IC0, Bit::D3),
        OutputExpanderSignal::KBD_S3 => Address(ShiftRegister::IC0, Bit::D4),
        OutputExpanderSignal::KBD_S4 => Address(ShiftRegister::IC0, Bit::D5),
        OutputExpanderSignal::KBD_S5 => Address(ShiftRegister::IC0, Bit::D6),
        OutputExpanderSignal::KBD_S6 => Address(ShiftRegister::IC0, Bit::D7),
        OutputExpanderSignal::KBD_S7 => Address(ShiftRegister::IC1, Bit::D0),
        OutputExpanderSignal::KBD_S8 => Address(ShiftRegister::IC1, Bit::D1),
        OutputExpanderSignal::SAIDA_VAGO1 => Address(ShiftRegister::IC1, Bit::D2),
        OutputExpanderSignal::COPIA_SINAL_PTR => Address(ShiftRegister::IC1, Bit::D3),
        OutputExpanderSignal::SAIDA_START_OUTRO => Address(ShiftRegister::IC1, Bit::D4),
        OutputExpanderSignal::INVMEN => Address(ShiftRegister::IC1, Bit::D5),
        OutputExpanderSignal::P3 => Address(ShiftRegister::IC1, Bit::D6),
        OutputExpanderSignal::P2 => Address(ShiftRegister::IC1, Bit::D7),
        OutputExpanderSignal::P1 => Address(ShiftRegister::IC2, Bit::D0),
        OutputExpanderSignal::P0 => Address(ShiftRegister::IC2, Bit::D1),
        OutputExpanderSignal::DMOTOR_1 => Address(ShiftRegister::IC2, Bit::D2),
        OutputExpanderSignal::DMOTOR_2 => Address(ShiftRegister::IC2, Bit::D3),
        OutputExpanderSignal::EMOTOR_1 => Address(ShiftRegister::IC2, Bit::D4),
        OutputExpanderSignal::EMOTOR_2 => Address(ShiftRegister::IC2, Bit::D5),
        OutputExpanderSignal::NMOTOR_1 => Address(ShiftRegister::IC2, Bit::D6),
        OutputExpanderSignal::NMOTOR_2 => Address(ShiftRegister::IC2, Bit::D7),
        OutputExpanderSignal::HF_1 => Address(ShiftRegister::IC3, Bit::D0),
        OutputExpanderSignal::HF_2 => Address(ShiftRegister::IC3, Bit::D1),
        OutputExpanderSignal::BUZZER => Address(ShiftRegister::IC3, Bit::D2),
        OutputExpanderSignal::LED_POS_ALC => Address(ShiftRegister::IC3, Bit::D3),
        OutputExpanderSignal::LED_PROGRAMA => Address(ShiftRegister::IC3, Bit::D4),
        OutputExpanderSignal::LED_ERRO => Address(ShiftRegister::IC3, Bit::D5),
        OutputExpanderSignal::LED_EXECUCAO => Address(ShiftRegister::IC3, Bit::D6),
        OutputExpanderSignal::LED_MANUAL => Address(ShiftRegister::IC3, Bit::D7),
    }
}

/// Responsible for send low latency output signals like keyboard, frontal panel leds, buzzer, etc.
pub struct OutputExpander {
    stage_area: Cell<ShiftOutData>,
    has_changed: Cell<bool>,
}

impl OutputExpander {
    /// NOTE: Do call this funcition just once in the entire program lifetime
    pub fn new() -> Self {
        init_shiftout_pins();
        OutputExpander {
            stage_area: Cell::new(ShiftOutData {
                byte0: 0x00,
                byte1: 0x00,
                byte2: 0x00,
                byte3: 0x00,
            }),
            has_changed: Cell::new(true), // first commit we must send all
        }
    }

    /// send all signais from the staged_area area to hardware if there is something to send
    pub fn commit(&self) -> () {
        // avoid to send data if nothing has changed
        if self.has_changed.get() {
            write_shiftout(self.stage_area.get());
            self.has_changed.set(false); //reset flag
        };
    }

    fn get_byte_from_stage_area__(&self, register: ShiftRegister) -> u8 {
        let stage_area = self.stage_area.get();
        match register {
            ShiftRegister::IC0 => stage_area.byte0,
            ShiftRegister::IC1 => stage_area.byte1,
            ShiftRegister::IC2 => stage_area.byte2,
            ShiftRegister::IC3 => stage_area.byte3,
        }
    }

    fn get_bit_from_stage_area__(&self, address: Address) -> bool {
        let Address(register, position) = address;
        let byte = self.get_byte_from_stage_area__(register);
        let bit = get_bit_at_as_bool(byte, position as u8);
        bit
    }

    fn set_bit_in_stage_area__(&self, address: Address, data_bit: bool) -> () {
        let Address(register, position) = address;
        let current_byte = self.get_byte_from_stage_area__(register);
        let new_byte = configure_bit(current_byte, position as u8, data_bit);
        // If there is nothing to modify than does nothing to avoid to unnecessary commits
        if new_byte != current_byte {
            self.has_changed.set(true);
            let mut data = self.stage_area.get();
            match register {
                ShiftRegister::IC0 => data.byte0 = new_byte,
                ShiftRegister::IC1 => data.byte1 = new_byte,
                ShiftRegister::IC2 => data.byte2 = new_byte,
                ShiftRegister::IC3 => data.byte3 = new_byte,
            };
            self.stage_area.set(data)
        };
    }

    fn stage_signal__(&self, signal: OutputExpanderSignal, data_bit: bool) -> &Self {
        let address = getAddress__(signal);
        self.set_bit_in_stage_area__(address, data_bit);
        self
    }

    // Public api

    pub fn KBD_SA(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::KBD_SA, data)
    }

    pub fn KBD_SB(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::KBD_SB, data)
    }

    pub fn KBD_S1(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::KBD_S1, data)
    }

    pub fn KBD_S2(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::KBD_S2, data)
    }

    pub fn KBD_S3(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::KBD_S3, data)
    }

    pub fn KBD_S4(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::KBD_S4, data)
    }

    pub fn KBD_S5(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::KBD_S5, data)
    }

    pub fn KBD_S6(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::KBD_S6, data)
    }

    pub fn KBD_S7(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::KBD_S7, data)
    }

    pub fn KBD_S8(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::KBD_S8, data)
    }

    pub fn SAIDA_VAGO1(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::SAIDA_VAGO1, data)
    }

    pub fn COPIA_SINAL_PTR(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::COPIA_SINAL_PTR, data)
    }

    pub fn SAIDA_START_OUTRO(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::SAIDA_START_OUTRO, data)
    }

    pub fn INVMEN(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::INVMEN, data)
    }

    pub fn P3(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::P3, data)
    }

    pub fn P2(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::P2, data)
    }

    pub fn P1(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::P1, data)
    }

    pub fn P0(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::P0, data)
    }

    pub fn DMOTOR_1(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::DMOTOR_1, data)
    }

    pub fn DMOTOR_2(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::DMOTOR_2, data)
    }

    pub fn EMOTOR_1(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::EMOTOR_1, data)
    }

    pub fn EMOTOR_2(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::EMOTOR_2, data)
    }

    pub fn NMOTOR_1(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::NMOTOR_1, data)
    }

    pub fn NMOTOR_2(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::NMOTOR_2, data)
    }

    pub fn HF_1(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::HF_1, data)
    }

    pub fn HF_2(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::HF_2, data)
    }

    pub fn BUZZER(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::BUZZER, data)
    }

    pub fn LED_POS_ALC(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::LED_POS_ALC, data)
    }

    pub fn LED_PROGRAMA(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::LED_PROGRAMA, data)
    }

    pub fn LED_ERRO(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::LED_ERRO, data)
    }

    pub fn LED_EXECUCAO(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::LED_EXECUCAO, data)
    }

    pub fn LED_MANUAL(&self, data: bool) -> &Self {
        self.stage_signal__(OutputExpanderSignal::LED_MANUAL, data)
    }
}

//

use crate::board::lcd::*;

pub fn development_entry_point() -> ! {
    lcd_initialize();
    print("iniciei");

    let output = OutputExpander::new();

    loop {
        output
            .BUZZER(true)
            .LED_ERRO(true)
            .LED_MANUAL(true)
            .LED_POS_ALC(true)
            .LED_PROGRAMA(true)
            .commit();

        delay_ms(2000);

        output
            .BUZZER(false)
            .LED_ERRO(false)
            .LED_MANUAL(false)
            .LED_POS_ALC(false)
            .LED_PROGRAMA(false)
            .commit();

        delay_ms(1000);
    }
}
