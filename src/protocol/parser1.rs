// separates ESC from ESC_DUP

use core::convert::TryInto;

use crate::board::lcd;

// NOTE: I'm abstraction away the 'u8' type into a 'Symbol' type because in future may be useful to define 'Symbol' in terms of another
// type. For example Symbol may be 'array of u8', instead of 'u8'. This would be useful for criptographic of compression purposes.
type Symbol = u8;   

const ESC:      Symbol = 0x1B;
const ESC_DUP:  Symbol = ESC;  // NOTE: This algoritmo allows you to choose another character to represent ESC_DUP
const STX:      Symbol = 0x02;
const ACK:      Symbol = 0x06;
const NACK:     Symbol = 0x15;
const ETX:      Symbol = 0x03;

struct Data {
    value: Symbol,
    is_esc_dup: bool,
}

enum Token {
    Esc,
    Data{
        value: Symbol,
        is_esc_dup: bool,
    },
    StartByte(Symbol),
    EndByte,
    Checksum(Symbol),
    Unknown(Symbol),
    Error(Symbol) //, &str),
}


struct Parser {
    last_was_esc: bool,
    last_was_etx: bool,
}

const buf_size:usize = 40; // 40 to represet at least one line of the 40 cols x 2 lines Lcd display
fn concat_str<'a>(a: &'a str, b: &'a str) -> [u8; buf_size] {
    let mut buf: [u8; buf_size] = [0; buf_size];  
    let len_a:usize = a.len();
    let start_b_index = (len_a-1) as usize; // buf index of the start of the second string
    let max_len_b:usize = buf_size-len_a;

    for i in 0..a.len() {
        buf[i] = a.as_bytes()[i];
    }

    for i in a.len()..(a.len()+b.len()) {

        buf[i] = b.as_bytes()[i-a.len()];         
    }
        
    

    //for i in 0..len_a {
    //    buf[i] = a.as_bytes()[i]; 
    //}

    //for i in 0..max_len_b {
    //    buf[i+start_b_index] = b.as_bytes()[i];
    //}

    buf

}

fn Token_to_str<'a>(data: Token) -> [u8; buf_size] {    
    match data {
        Token::Esc                      => concat_str("Esc;",""),
        Token::Data{value, is_esc_dup}  => concat_str("Data;", ""),
        Token::StartByte(value)         => concat_str("StartByte;",""),
        Token::EndByte                  => concat_str("EndByte;",""),
        Token::Checksum(value)          => concat_str("CheckSum;",""),
        Token::Unknown(value)           => concat_str("Unknown;",""),
        Token::Error(value)             => concat_str("Error;",""),
        _                               => concat_str("Unknown;",""),
    }
}

impl Parser {
    fn new() -> Self {
        Parser {
            last_was_esc: false,
            last_was_etx: false,
        }
    }

    fn parse_next(&mut self, byte: u8) -> Token {
        if self.last_was_etx == true {
            self.last_was_etx = false; // reset flag ETX
            Token::Checksum(byte)
        } else {
            if self.last_was_esc == true {
                self.last_was_esc = false; // reset flag ESC
                //look now for esc_dup or instructionlet
                match byte {
                    STX => {
                        Token::StartByte(STX)
                    },
                    ACK => {
                        Token::StartByte(ACK)
                    },
                    NACK => {
                        Token::StartByte(NACK)
                    },
                    ETX => {
                        self.last_was_etx = true; // set flag ETX
                        Token::EndByte
                    },
                    ESC_DUP => {
                        self.last_was_esc = false;
                        Token::Data{value: byte, is_esc_dup: true}
                    },
                    _ => {
                        Token::Error(byte) //, "Erro: ESC nao acompanhado de start ou end byte valido, e nem de um esc duplicado")
                    },
                }
            } else { // self.last_was_esc == false
                match byte {
                    ESC => {
                        self.last_was_esc = true; // set flag ESC
                        Token::Esc
                    },
                    _ => {
                        Token::Data{value: byte, is_esc_dup: false}
                    }
                }
    
            }
        }
    }
}


//

pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();

    lcd::print("-> ");

    let mut parser = Parser::new();

    lcd::print_u8_array(Token_to_str(parser.parse_next(ESC)));
    lcd::print_u8_array(Token_to_str(parser.parse_next(0)));
    lcd::print_u8_array(Token_to_str(parser.parse_next(ESC)));
    lcd::print_u8_array(Token_to_str(parser.parse_next(STX)));
    lcd::print_u8_array(Token_to_str(parser.parse_next(ESC)));
    lcd::print_u8_array(Token_to_str(parser.parse_next(ESC_DUP)));
    lcd::print_u8_array(Token_to_str(parser.parse_next(0)));
    lcd::print_u8_array(Token_to_str(parser.parse_next(0)));
    lcd::print_u8_array(Token_to_str(parser.parse_next(0)));
    lcd::print_u8_array(Token_to_str(parser.parse_next(ESC)));
    lcd::print_u8_array(Token_to_str(parser.parse_next(ETX)));
    lcd::print_u8_array(Token_to_str(parser.parse_next(20)));
    //lcd::print_u8_array(Token_to_str(parser.parse_next(ESC)));
    
    


    loop {


    }

}


