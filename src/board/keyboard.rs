// low-level driver for keypad

use arduino_hal::pac::AC;

use crate::{board::lcd, common::set_bit_at};

use super::{shiftout::{write_shiftout, ShiftOutData}, shiftin::readShiftIn, output_expander::OutputExpander, input_expander::InputExpander};

use crate::microcontroler::delay::delay_ms;

const ACTIVATED: bool = false; //low level
const DEACTIVATE: bool = true; //true level

// 

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum KeyCode {
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

//constants

struct Keypad { 
    output: OutputExpander,
    input: InputExpander,
    //last_keycode_read: KeyCode,
}

const keymap: [[KeyCode;8];4] = [
    [KeyCode::KEY_F1,  KeyCode::KEY_7,  KeyCode::KEY_8,  KeyCode::KEY_9,       KeyCode::KEY_EXECUCAO,               KeyCode::KEY_INS,                       KeyCode::KEY_ESC,                        KeyCode::KEY_HIDDEN_KEY],
    [KeyCode::KEY_F2,  KeyCode::KEY_4,  KeyCode::KEY_5,  KeyCode::KEY_6,       KeyCode::KEY_MAIS_OU_MENOS,          KeyCode::KEY_DIRECIONAL_PARA_ESQUERDA,  KeyCode::KEY_DIRECIONAL_PARA_CIMA,       KeyCode::KEY_START],
    [KeyCode::KEY_F3,  KeyCode::KEY_1,  KeyCode::KEY_2,  KeyCode::KEY_3,       KeyCode::KEY_SETA_BRANCA_DIREITA,    KeyCode::KEY_DIRECIONAL_PARA_BAIXO,     KeyCode::KEY_DIRECIONAL_PARA_DIREITA,    KeyCode::KEY_MANUAL],
    [KeyCode::KEY_F4,  KeyCode::KEY_0,  KeyCode::KEY_0,  KeyCode::KEY_ENTER,   KeyCode::KEY_SETA_BRANCA_ESQUERDA,   KeyCode::KEY_DEL,                       KeyCode::KEY_STOP,                       KeyCode::KEY_PROGRAMA],
];


impl Keypad {

    pub fn new() -> Self {
        let output = OutputExpander::new();
        let input = InputExpander::new();
        Keypad { 
            output,
            input,
        }
    }

    fn set_output(&mut self, n: u8, value: bool) -> () {
        //ATTENTION: Do call commit() after write.
        match n {
            0 => self.output.KBD_S1(value).commit(),
            1 => self.output.KBD_S2(value).commit(),
            2 => self.output.KBD_S3(value).commit(),
            3 => self.output.KBD_S4(value).commit(),
            4 => self.output.KBD_S5(value).commit(),
            5 => self.output.KBD_S6(value).commit(),
            6 => self.output.KBD_S7(value).commit(),
            7 => self.output.KBD_S8(value).commit(),
            _ => unreachable!(),
        };
    }

    fn get_input(&mut self, n: u8) -> bool {
        //ATTENTION: Do call fetch() before read.
        match n {
            0 => self.input.fetch().KBD_E1(),
            1 => self.input.fetch().KBD_E2(),
            2 => self.input.fetch().KBD_E3(),
            3 => self.input.fetch().KBD_E4(),
            4 => self.input.fetch().KBD_E5(),
            5 => self.input.fetch().KBD_E6(),
            6 => self.input.fetch().KBD_E7(),
            7 => self.input.fetch().KBD_E8(),
            _ => unreachable!(),            
        }
    }

    fn scan(&mut self) -> KeyCode {
        let mut key_code: KeyCode = KeyCode::NO_KEY;       
        for collumn in 0..=7 {
            self.set_output(collumn, ACTIVATED);
            for row in 0..=3 {
                let bit = self.get_input(row);
                if bit == ACTIVATED {
                    key_code = keymap[row as usize][collumn as usize];
                }
            }
            self.set_output(collumn, DEACTIVATE);
        };
        key_code
  
    }


}



pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();
    lcd::clear();
    lcd::print("Pressione qualquer tecla");

    let mut keypad = Keypad::new();

    loop {
        let keycode = keypad.scan();
        lcd::setCursor(0, 1);
        lcd::print_u8_in_hex(keycode as u8);
        delay_ms(100);
    }

}






