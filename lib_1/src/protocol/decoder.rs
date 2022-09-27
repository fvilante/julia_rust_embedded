#[allow(unused_variables)]

use super::{common::*, checksum::calc_checksum};
use super::frame::Frame as Frame2;

const MAX_BUFFER_LEN: usize = 4; // max data length buffer

#[derive(Debug)]
pub enum SegmentError {
    InvalidStartByte(u8),
    BufferOverFlow,
    ExpectedEtxOrEscDupButFoundOtherThing(u8),
    ChecksumIsEscButNotDuplicated(u8),
    InvalidChecksum { expected: u8, received: u8 },
}

impl SegmentError {
   
    pub fn to_string(&self) -> &'static str {
        match *self {
            SegmentError::InvalidStartByte(_u8) => "InvalidStartByte",
            SegmentError::BufferOverFlow => "BufferOverFlow",
            SegmentError::ExpectedEtxOrEscDupButFoundOtherThing(_u8) => "ExpectedEtxOrEscDupButFoundOtherThing",
            SegmentError::ChecksumIsEscButNotDuplicated(_u8) => "ChecksumIsEscButNotDuplicated",
            SegmentError::InvalidChecksum { expected: _a, received: _b } => "InvalidChecksum",
        }
    }
}


pub struct SegmentResult {
    pub start_byte: StartByte,
    pub frame: Frame,
}


pub enum State {
    WaitingFirstEsc,
    WaitingStartByte,
    ReceivingData,
    WaitingChecksum,
}

pub struct Decoder {
    start_byte: StartByte,
    state: State,
    buffer_index: usize,
    buffer: [u8;MAX_BUFFER_LEN],
    last_was_esc: bool,
}

impl Decoder {

    pub fn new() -> Self {
        Self {
            start_byte: StartByte::STX,
            state: State::WaitingFirstEsc,
            buffer_index: 0,
            buffer: [0x00; MAX_BUFFER_LEN],
            last_was_esc: false,
        }
    }

    fn save_data(&mut self, data: u8) -> Result<(),SegmentError> {
        if self.buffer_index < self.buffer.len() {
            self.buffer[self.buffer_index] = data;
            self.buffer_index += 1;
            Ok(())
        } else {
            Err(SegmentError::BufferOverFlow)
        }
        
    }

    fn success(&self, checksum: u8) -> Result<Option<SegmentResult>, SegmentError> {
       
        let frame = Frame2{
            start_byte: self.start_byte,
            payload: self.buffer ,
        };
        let expected = calc_checksum(frame);
        if checksum == expected {
            Ok(Some( SegmentResult { 
                start_byte: self.start_byte,
                frame: Frame::from_array(&frame.payload), 
            }))
        } else {
            Err(SegmentError::InvalidChecksum { expected, received: (checksum) })
        }
        
    }

    pub fn parse_next(&mut self, byte: u8) -> Result<Option<SegmentResult>, SegmentError> {
        match self.state {
            State::WaitingFirstEsc => {
                self.state = State::WaitingStartByte;
                Ok(None)
            }

            State::WaitingStartByte => {
                let start_byte: Result<StartByte, SegmentError> = match byte {
                    STX     => Ok(StartByte::STX),
                    ACK     => Ok(StartByte::ACK),
                    NACK    => Ok(StartByte::NACK),
                    _       => Err(SegmentError::InvalidStartByte(byte)),
                };
                match start_byte {
                    Ok(valid) => {
                        self.state = State::ReceivingData;
                        self.start_byte = valid;
                        Ok(None)
                    }

                    Err(e) => Err(e),
                }
             
            }

            State::ReceivingData => {

                if self.last_was_esc {
                    if byte == ESC {
                        //escdup
                        self.last_was_esc = false;
                        match self.save_data(ESC) {
                           Ok(_)    => Ok(None),
                           Err(e)   => Err(e),  
                        }
                    } else if byte == ETX {
                        //etx
                        self.last_was_esc = false;
                        self.state = State::WaitingChecksum;
                        Ok(None)    
                    } else {
                        Err(SegmentError::ExpectedEtxOrEscDupButFoundOtherThing(byte))
                    }
                } else {
                    if byte == ESC {
                        self.last_was_esc = true;
                        Ok(None)
                    } else {
                        //normal data
                        match self.save_data(byte) {
                            Ok(_)       => Ok(None),
                            Err(e)   => Err(e),  
                        }
                    }
                }
            }

            State::WaitingChecksum => {
                
                if self.last_was_esc {
                    if byte == ESC {
                        //Escdup
                        self.last_was_esc = false;
                        let checksum = ESC;
                        self.success(checksum)
                    } else {
                        Err(SegmentError::ChecksumIsEscButNotDuplicated(byte))
                    }
                } else {
                    if byte == ESC {
                        self.last_was_esc = true;
                        Ok(None)
                    } else {
                        // non-esc checksum
                        self.last_was_esc = false;
                        let checksum = byte;
                        self.success(checksum)
                    }
                }
            }
        }
    }
}

pub fn add(left: u8, right: u8) -> u8 {
    left + right + 2
}

#[cfg(test)]
mod tests {

    use super::*;

    fn run_decoder(input_probe: &[u8]) ->  Result<SegmentResult, SegmentError> { 
        let mut decoder = Decoder::new();
        for byte in input_probe {
            let result = decoder.parse_next(*byte);
            match result {
                Ok(val) => {
                    match val  {
                        Some(segment) => return Ok(segment),
                        None => { /* nop */ }
                    }
                }

                Err(e) => return Err(e),
            }
        }
        unreachable!()
    }

    #[test]
    fn it_parse_a_segment() {
        // 1B 02 C1 50 61 02 1B 03 87 
        let start_byte__ = StartByte::STX; 
        let input_probe = [0x1B, start_byte__ as u8, 0xC1, 0x50, 0x61, 0x02, 0x1B, 0x03, 0x87, ];
        let expected = Frame(0xC1, 0x50, 0x61, 0x02,);
        if let Ok(segment) = run_decoder(&input_probe) {
            let SegmentResult { start_byte, frame }= segment;
            assert_eq!(expected, frame);
            assert_eq!(start_byte, start_byte__);
        } else {
            assert_eq!(true, false); // Unexpected behaviour, Returned an SegmentError
        }
    }

    #[test]
    fn it_parse_a_segment_with_esc_dup() {
        // 1B 06 01 86 03 1B 1B 03 52 
        let start_byte__ = StartByte::ACK; 
        let input_probe = [0x1B, 0x06, 0x01, 0x86, 0x03, 0x1B, 0x1B, 0x1B, 0x03, 0x52 ];
        let expected = Frame(0x01, 0x86, 0x03, 0x1B,);
        if let Ok(segment) = run_decoder(&input_probe) {
            let SegmentResult { start_byte, frame }= segment;
            assert_eq!(expected, frame);
            assert_eq!(start_byte, start_byte__);
        } else {
            assert_eq!(true, false); // Unexpected behaviour, Returned an SegmentError
        }
    }

}
