use super::frame::Frame;
use super::{
    checksum::calc_checksum,
    prelude::{StartByte, ACK, ESC, ETX, NACK, STX},
};

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
            SegmentError::ExpectedEtxOrEscDupButFoundOtherThing(_u8) => {
                "ExpectedEtxOrEscDupButFoundOtherThing"
            }
            SegmentError::ChecksumIsEscButNotDuplicated(_u8) => "ChecksumIsEscButNotDuplicated",
            SegmentError::InvalidChecksum {
                expected: _a,
                received: _b,
            } => "InvalidChecksum",
        }
    }
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
    buffer: [u8; MAX_BUFFER_LEN],
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

    fn save_data(&mut self, data: u8) -> Result<(), SegmentError> {
        if self.buffer_index < self.buffer.len() {
            self.buffer[self.buffer_index] = data;
            self.buffer_index += 1;
            Ok(())
        } else {
            Err(SegmentError::BufferOverFlow)
        }
    }

    fn success(&self, checksum: u8) -> Result<Option<Frame>, SegmentError> {
        let frame = Frame {
            start_byte: self.start_byte,
            payload: self.buffer,
        };
        let expected = calc_checksum(frame);
        if checksum == expected {
            Ok(Some(frame))
        } else {
            Err(SegmentError::InvalidChecksum {
                expected,
                received: (checksum),
            })
        }
    }

    pub fn parse_next(&mut self, byte: u8) -> Result<Option<Frame>, SegmentError> {
        match self.state {
            State::WaitingFirstEsc => {
                self.state = State::WaitingStartByte;
                Ok(None)
            }

            State::WaitingStartByte => {
                let start_byte: Result<StartByte, SegmentError> = match byte {
                    STX => Ok(StartByte::STX),
                    ACK => Ok(StartByte::ACK),
                    NACK => Ok(StartByte::NACK),
                    _ => Err(SegmentError::InvalidStartByte(byte)),
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
                            Ok(_) => Ok(None),
                            Err(e) => Err(e),
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
                            Ok(_) => Ok(None),
                            Err(e) => Err(e),
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

#[cfg(test)]
mod tests {

    use crate::protocol::frame::Payload;

    use super::*;

    fn run_decoder(input: Payload) -> Result<Frame, SegmentError> {
        let mut decoder = Decoder::new();
        for byte in input {
            match decoder.parse_next(byte) {
                Ok(val) => {
                    match val {
                        Some(frame) => return Ok(frame),
                        None => { /* nop */ }
                    }
                }

                Err(e) => return Err(e),
            }
        }
        unreachable!()
    }

    #[test]
    fn it_parse_a_frame() {
        // 1B 02 C1 50 61 02 1B 03 87
        let start_byte = StartByte::STX;
        let start_byte_ = start_byte as u8;
        let probe = [0x1B, start_byte_, 0xC1, 0x50, 0x61, 0x02, 0x1B, 0x03, 0x87];
        let expected = Frame {
            start_byte,
            payload: [0xC1, 0x50, 0x61, 0x02],
        };
        let actual = run_decoder(probe).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_parse_a_segment_with_esc_dup() {
        // 1B 06 01 86 03 1B 1B 03 52
        let start_byte = StartByte::ACK;
        let start_byte_ = start_byte as u8;
        let probe = [
            0x1B,
            start_byte_,
            0x01,
            0x86,
            0x03,
            0x1B,
            0x1B,
            0x1B,
            0x03,
            0x52,
        ];
        let expected = Frame {
            start_byte,
            payload: [0x01, 0x86, 0x03, 0x1B],
        };
        let actual = run_decoder(probe).unwrap();
        assert_eq!(expected, actual);
    }
}
