// mid-level abstraction for on-board shift registers input expanders

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use lib_1::utils::common::get_bit_at_as_bool;

use super::shiftin::{ShiftInData, self, readShiftIn};

use crate::board::lcd::{
    lcd_initialize,
    print_u8_in_hex,
};

// Represents each of the three CD4021 Integrated Circuit present on the board
enum ShiftRegister {
    IC0,  // Board descriptor: U109 
    IC1,  // Board descriptor: U102
    IC2   // Board descriptor: U106
}


enum Bit {
    D0,     // bit 0 of a byte
    D1,     // bit 1 of a byte
    D2,     // etc...
    D3,
    D4,
    D5,
    D6,
    D7,
}

// Address of the input signal
struct Address(ShiftRegister, Bit);

enum InputExpanderSignalRequest {
    START,
    FC_MAIS_1,
    FC_MAIS_2,
    ENTRADA_START_OUTRO,
    EMERG,
    BUSY,
    FC_MENOS_2,
    FC_MENOS_1,
    KBD_E1,
    KBD_E2,
    KBD_E3,
    KBD_E4,
    KBD_E5,
    KBD_E6,
    KBD_E7,
    KBD_E8,
    REF_1,
    REF_2,
    ENTRADA_VAGO1,
    ENTRADA_VAGO2,
    //BELLOW ARE INPUTs ARE NOT IN USE AND ELETRICALLY CONNECTED TO THE GROUND (LOW LEVEL).
    INPUT_BUS20,
    INPUT_BUS21,
    INPUT_BUS22,
    INPUT_BUS23,
}

fn get_adddress(signal: InputExpanderSignalRequest) -> Address {
    match signal {
        InputExpanderSignalRequest::START                   => Address(ShiftRegister::IC0, Bit::D0),
        InputExpanderSignalRequest::FC_MAIS_1               => Address(ShiftRegister::IC0, Bit::D1),
        InputExpanderSignalRequest::FC_MAIS_2               => Address(ShiftRegister::IC0, Bit::D2),
        InputExpanderSignalRequest::ENTRADA_START_OUTRO     => Address(ShiftRegister::IC0, Bit::D3),
        InputExpanderSignalRequest::EMERG                   => Address(ShiftRegister::IC0, Bit::D4),
        InputExpanderSignalRequest::BUSY                    => Address(ShiftRegister::IC0, Bit::D5),
        InputExpanderSignalRequest::FC_MENOS_2              => Address(ShiftRegister::IC0, Bit::D6),
        InputExpanderSignalRequest::FC_MENOS_1              => Address(ShiftRegister::IC0, Bit::D7),
        InputExpanderSignalRequest::KBD_E1                  => Address(ShiftRegister::IC1, Bit::D0),
        InputExpanderSignalRequest::KBD_E2                  => Address(ShiftRegister::IC1, Bit::D1),
        InputExpanderSignalRequest::KBD_E3                  => Address(ShiftRegister::IC1, Bit::D2),
        InputExpanderSignalRequest::KBD_E4                  => Address(ShiftRegister::IC1, Bit::D3),
        InputExpanderSignalRequest::KBD_E5                  => Address(ShiftRegister::IC1, Bit::D4),
        InputExpanderSignalRequest::KBD_E6                  => Address(ShiftRegister::IC1, Bit::D5),
        InputExpanderSignalRequest::KBD_E7                  => Address(ShiftRegister::IC1, Bit::D6),
        InputExpanderSignalRequest::KBD_E8                  => Address(ShiftRegister::IC1, Bit::D7),
        InputExpanderSignalRequest::REF_1                   => Address(ShiftRegister::IC2, Bit::D0),
        InputExpanderSignalRequest::REF_2                   => Address(ShiftRegister::IC2, Bit::D1),
        InputExpanderSignalRequest::ENTRADA_VAGO1           => Address(ShiftRegister::IC2, Bit::D2),
        InputExpanderSignalRequest::ENTRADA_VAGO2           => Address(ShiftRegister::IC2, Bit::D3),
        InputExpanderSignalRequest::INPUT_BUS20             => Address(ShiftRegister::IC2, Bit::D4),
        InputExpanderSignalRequest::INPUT_BUS21             => Address(ShiftRegister::IC2, Bit::D5),
        InputExpanderSignalRequest::INPUT_BUS22             => Address(ShiftRegister::IC2, Bit::D6),
        InputExpanderSignalRequest::INPUT_BUS23             => Address(ShiftRegister::IC2, Bit::D7),
    }
}



pub struct InputExpander {
    cache: ShiftInData,
    is_first_run: bool,
}

impl InputExpander {

    pub fn new() -> Self {
        Self {
            cache: ShiftInData {
                byte0: 0x00,
                byte1: 0x00,
                byte2: 0x00,
            },
            is_first_run: true,
        }
    }

    // fetch data from the hardware and save it on memory cache
    pub fn fetch(&mut self) -> &mut Self {
        let data_read = readShiftIn();
        self.cache = data_read;
        self
    }

    // NOTE: If first run fetch data from hardware else from cache.
    //       To pull data from hardware use 'fetch' method.
    fn get_signal__(&mut self, signal: InputExpanderSignalRequest) -> bool {
        let cache = {
            if self.is_first_run == true {
                self.is_first_run = false;
                &self.fetch().cache
            } else {
                &self.cache
            }
        };
        let Address(register, position) = get_adddress(signal);
        let byte = match register {
            ShiftRegister::IC0 => cache.byte0,
            ShiftRegister::IC1 => cache.byte1,
            ShiftRegister::IC2 => cache.byte2,
        };
        let bit = get_bit_at_as_bool(byte, position as u8);
        bit
    }


    // Public api 

    pub fn START(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::START)
    }
    pub fn FC_MAIS_1(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::FC_MAIS_1)
    }
    pub fn FC_MAIS_2(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::FC_MAIS_2)
    }
    pub fn ENTRADA_START_OUTRO(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::ENTRADA_START_OUTRO)
    }
    pub fn EMERG(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::EMERG)
    }
    pub fn BUSY(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::BUSY)
    }
    pub fn FC_MENOS_2(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::FC_MENOS_2)
    }
    pub fn FC_MENOS_1(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::FC_MENOS_1)
    }
    pub fn KBD_E1(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::KBD_E1)
    }
    pub fn KBD_E2(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::KBD_E2)
    }
    pub fn KBD_E3(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::KBD_E3)
    }
    pub fn KBD_E4(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::KBD_E4)
    }
    pub fn KBD_E5(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::KBD_E5)
    }
    pub fn KBD_E6(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::KBD_E6)
    }
    pub fn KBD_E7(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::KBD_E7)
    }
    pub fn KBD_E8(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::KBD_E8)
    }
    pub fn REF_1(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::REF_1)
    }
    pub fn REF_2(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::REF_2)
    }
    pub fn ENTRADA_VAGO1(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::ENTRADA_VAGO1)
    }
    pub fn ENTRADA_VAGO2(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::ENTRADA_VAGO2)
    }
    pub fn INPUT_BUS20(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::INPUT_BUS20)
    }
    pub fn INPUT_BUS21(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::INPUT_BUS21)
    }
    pub fn INPUT_BUS22(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::INPUT_BUS22)
    }
    pub fn INPUT_BUS23(&mut self) -> bool {
        self.get_signal__(InputExpanderSignalRequest::INPUT_BUS23)
    }
} 


//

pub fn development_entry_point() -> ! {

    let mut input = InputExpander::new();

    // retrieve data from hardware to cache
    input.fetch();

    // read from cache
    let (d0,d1,d2,d3,d4,d5,d6,d7) = (
        input.KBD_E1(), 
        input.KBD_E2(),
        input.KBD_E3(),
        input.KBD_E4(),
        input.KBD_E5(),
        input.KBD_E6(),
        input.KBD_E7(),
        input.KBD_E8(),
    );

    let value:u8 = 
        (d0 as u8) * (1 << 0) +
        (d1 as u8) * (1 << 1) +
        (d2 as u8) * (1 << 2) +
        (d3 as u8) * (1 << 3) +
        (d4 as u8) * (1 << 4) +
        (d5 as u8) * (1 << 5) +
        (d6 as u8) * (1 << 6) +
        (d7 as u8) * (1 << 7);
    

    lcd_initialize();
    print_u8_in_hex(value);


    loop { }

}