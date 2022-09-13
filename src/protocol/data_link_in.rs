// Process low-level input stream using Posijet Version 1.0 protocol

use crate::board::lcd::{self, lcd_initialize};


const Esc:  u8 = 0x1B;
const Stx:  u8 = 0x02;
const Ack:  u8 = 0x06;
const Nack: u8 = 0x15;
const Etx:  u8 = 0x03;


enum Token1 {
    ESC,
    ESC_DUP,
    UNKNOWN(u8),  // any byte that is not ESC nor ESC_DUP
}

fn Token1_to_str(data: Token1) -> &'static str {
    match data {
        Token1::ESC             => "Esc",
        Token1::ESC_DUP         => "Esc_Dup",
        Token1::UNKNOWN(value)  => "Unknown",
     }
}

enum StartByte {
    STX,
    ACK,
    NACK,
}

enum EndByte {
    ETX
}

enum Token2 {
    ESC,
    StartByte,
    Data(u8,u8,u8,u8),
    ACK,
    NACK,
    ETX,
    CHECKSUM(u8),
}


struct Parser1 { 
    last_was_esc: bool  // if last byte was esc then true
}

impl Parser1 {

    fn new() -> Self {
        Parser1 { 
            last_was_esc: false 
        }
    }

    fn process_next(&mut self, byte: u8) -> Token1 {
        match byte {
            Esc => {
                if self.last_was_esc == true {
                    self.last_was_esc = false;
                    Token1::ESC_DUP
                } else {
                    self.last_was_esc = true;
                    Token1::ESC
                }
            },
            Ack => {
                Token1::UNKNOWN(byte)
            }
        }
    }
}

//struct Parser_2 {
//    
//}
//
//impl Parser_2 {
//    fn process_next(&mut self, data: Token_1) -> Token_2 {
//        match data {
//
//        }
//    }
//}



//

pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();

    lcd::print("Protocol v1.0.0");

    let mut parser = Parser1::new();

    lcd::print(&Token1_to_str(parser.process_next(Esc)));
    lcd::print(&Token1_to_str(parser.process_next(Esc)));
    lcd::print(&Token1_to_str(parser.process_next(Esc)));
    lcd::print(&Token1_to_str(parser.process_next(Esc)));
    lcd::print(&Token1_to_str(parser.process_next(Esc)));
    lcd::print(&Token1_to_str(parser.process_next(Esc)));

    
    lcd::print(&Token1_to_str(parser.process_next(0)));
    lcd::print(&Token1_to_str(parser.process_next(1)));
    lcd::print(&Token1_to_str(parser.process_next(2)));
    loop {


    }

}



