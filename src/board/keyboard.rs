// on-board circuit keyboard driver

#![allow(non_camel_case_types)]

use ruduino::Pin;
use ruduino::cores::atmega328p::{port};

use crate::board::lcd;
use crate::microcontroler::delay::delay_us;

const HIGH: bool = true;
const LOW: bool = false;

enum KEY_CODES {
    //Some key codes try to imitate ASCII table codes (ie: ESC, Enter and numerals digitis )
    //Other codes on this table was arbitrary assigned.
    //Key codes are grouped by family of keys

    //If no key has been pressed yet
    NO_KEY = 0x00,

    //flow keys
    KEY_ENTER = 0x0A,
    KEY_ESC = 0x1B,

    //edition keys
    KEY_SETA_BRANCA_ESQUERDA = 0x20, 
    KEY_SETA_BRANCA_DIREITA = 0x21,
    KEY_MAIS_OU_MENOS = 0x2D,

    //printable keys
    KEY_PONTO = 0x2E,
    KEY_0 = 0x30,
    KEY_1 = 0x31,
    KEY_2 = 0x32,
    KEY_3 = 0x33,
    KEY_4 = 0x34,
    KEY_5 = 0x35,
    KEY_6 = 0x36,
    KEY_7 = 0x37,
    KEY_8 = 0x38,
    KEY_9 = 0x39,

    //execution keys
    KEY_START = 0x80,
    KEY_STOP = 0x81,

    //menu keys
    KEY_MANUAL = 0x82,
    KEY_EXECUCAO = 0x83,
    KEY_PROGRAMA = 0x84,

    //function keys
    KEY_F1 = 0x85,
    KEY_F2 = 0x86,
    KEY_F3 = 0x87,
    KEY_F4 = 0x88,

    //direction keys
    KEY_DIRECIONAL_PARA_CIMA = 0x89,
    KEY_DIRECIONAL_PARA_BAIXO = 0x8A,
    KEY_DIRECIONAL_PARA_DIREITA = 0x8B,
    KEY_DIRECIONAL_PARA_ESQUERDA = 0x8C,

    //registry keys
    KEY_INS = 0x8D,
    KEY_DEL = 0x8E,

    //control key
    KEY_CTRL = 0x8F,        //NOTE: Before use KEY_CTRL verify if it is implemented in hardware.

    //secret key
    KEY_HIDDEN_KEY = 0x90,    
}


struct KeypadSize {
    rows: u8,
    collumns: u8
}

enum KeypadState {
    IDLE=0,
    PRESSED,
    RELEASED,
    HOLD,
}

enum INPUT_PIN {
    KBD_E1,
    KBD_E2,
    KBD_E3,
    KBD_E4,
}

enum OUTPUT_PIN {
    KBD_S1(bool),
    KBD_S2(bool),
    KBD_S3(bool),
    KBD_S4(bool),
    KBD_S5(bool),
    KBD_S6(bool),
    KBD_S7(bool),
    KBD_S8(bool),
}

    

fn set_output(pin: OUTPUT_PIN) -> () {
    match pin {
        OUTPUT_PIN::KBD_S1(value) => unimplemented!(),
        OUTPUT_PIN::KBD_S2(value) => unimplemented!(),
        OUTPUT_PIN::KBD_S3(value) => unimplemented!(),
        OUTPUT_PIN::KBD_S4(value) => unimplemented!(),
        OUTPUT_PIN::KBD_S5(value) => unimplemented!(),
        OUTPUT_PIN::KBD_S6(value) => unimplemented!(),
        OUTPUT_PIN::KBD_S7(value) => unimplemented!(),
        OUTPUT_PIN::KBD_S8(value) => unimplemented!(),
    }
}

fn read_input(pin: INPUT_PIN) -> bool {
    match pin {
        INPUT_PIN::KBD_E1 => unimplemented!(),
        INPUT_PIN::KBD_E2 => unimplemented!(),
        INPUT_PIN::KBD_E3 => unimplemented!(),
        INPUT_PIN::KBD_E4 => unimplemented!(),
    }
}

//constants
const keymap: [[KEY_CODES;4];8] = [
    [KEY_CODES::KEY_F1,  KEY_CODES::KEY_7,  KEY_CODES::KEY_8,  KEY_CODES::KEY_9,       KEY_CODES::KEY_EXECUCAO,               KEY_CODES::KEY_INS,                       KEY_CODES::KEY_ESC,                        KEY_CODES::KEY_HIDDEN_KEY],
    [KEY_CODES::KEY_F2,  KEY_CODES::KEY_4,  KEY_CODES::KEY_5,  KEY_CODES::KEY_6,       KEY_CODES::KEY_MAIS_OU_MENOS,          KEY_CODES::KEY_DIRECIONAL_PARA_ESQUERDA,  KEY_CODES::KEY_DIRECIONAL_PARA_CIMA,       KEY_CODES::KEY_START],
    [KEY_CODES::KEY_F3,  KEY_CODES::KEY_1,  KEY_CODES::KEY_2,  KEY_CODES::KEY_3,       KEY_CODES::KEY_SETA_BRANCA_DIREITA,    KEY_CODES::KEY_DIRECIONAL_PARA_BAIXO,     KEY_CODES::KEY_DIRECIONAL_PARA_DIREITA,    KEY_CODES::KEY_MANUAL],
    [KEY_CODES::KEY_F4,  KEY_CODES::KEY_0,  KEY_CODES::KEY_0,  KEY_CODES::KEY_ENTER,   KEY_CODES::KEY_SETA_BRANCA_ESQUERDA,   KEY_CODES::KEY_DEL,                       KEY_CODES::KEY_STOP,                       KEY_CODES::KEY_PROGRAMA],
];
const ROWS: u8 = 8; // eight rows (inputs)
const COLS: u8 = 8; // eight columns (outputs)
const size: KeypadSize = KeypadSize { rows: ROWS, collumns: COLS };
const rowPins: [INPUT_PIN;4] = [ 
    INPUT_PIN::KBD_E1, 
    INPUT_PIN::KBD_E2, 
    INPUT_PIN::KBD_E3, 
    INPUT_PIN::KBD_E4 
];
const colPins: [OUTPUT_PIN;8] = [ 
    OUTPUT_PIN::KBD_S1,
    OUTPUT_PIN::KBD_S2,
    OUTPUT_PIN::KBD_S3,
    OUTPUT_PIN::KBD_S4,
    OUTPUT_PIN::KBD_S5,
    OUTPUT_PIN::KBD_S6,
    OUTPUT_PIN::KBD_S7,
    OUTPUT_PIN::KBD_S8,
];
const DEACTIVATE: u8 = HIGH;
const ACTIVATE: u8 = LOW;

//keyboard internal state
static state: KeypadState = KeypadState::IDLE;
static currentKey: KEY_CODES = KEY_CODES::NO_KEY;
static lastUpdate: u64 = 0;    // milisecs?
static debounceTime: u32 = 50; // microseconds
static holdTime: u32 = 1000; // milisecs?

fn init_keyboard() -> () {

    for each_pin in colPins {
        set_output(each_pin(DEACTIVATE));
    };

}

// Returns the key code of the pressed key, or NO_KEY if no key is pressed
fn getKey() -> KEY_CODES {

    //assume that no key is pressed, this is the default return for getKey()
    let key: KEY_CODES = KEY_CODES::NO_KEY;

        for col_pin in OUTPUT_PIN {
            set_output(col_pin(ACTIVATE)); // Activate the current collumn
            for row_pin in INPUT_PIN {
                key_read = read_input(row_pin);
                if key_read == true {
                    //currentKey = keymap[]
                    Beep();
                }

            }
            set_output(col_pin(DEACTIVATE));

        }




}




// 

pub fn entry_point_for_development() -> ! {

}