use super::frame::Frame;
use super::{
    checksum::calc_checksum,
    prelude::{StartByte, ACK, ESC, ETX, NACK, STX},
};

const MAX_BUFFER_LEN: usize = 4; // max data length buffer

#[derive(Debug)]
pub enum DecodingError {
    InvalidStartByte(u8),
    BufferOverFlow,
    ExpectedEtxOrEscDupButFoundOtherThing(u8),
    ChecksumIsEscButNotDuplicated(u8),
    InvalidChecksum { expected: u8, received: u8 },
}

#[derive(PartialEq)]
pub enum State {
    WaitingFirstEsc,
    WaitingStartByte,
    ReceivingData,
    WaitingChecksum,
}

pub struct Decoder {
    start_byte: StartByte,
    state: State,
    payload_index: usize,
    payload_buffer: [u8; MAX_BUFFER_LEN],
    last_was_esc: bool,
}

impl Decoder {
    pub const fn new() -> Self {
        Self {
            start_byte: StartByte::STX,
            state: State::WaitingFirstEsc,
            payload_index: 0,
            payload_buffer: [0x00; MAX_BUFFER_LEN],
            last_was_esc: false,
        }
    }

    pub fn reset(&mut self) {
        self.start_byte = StartByte::STX;
        self.state = State::WaitingFirstEsc;
        self.payload_index = 0;
        self.payload_buffer = [0x00; MAX_BUFFER_LEN];
        self.last_was_esc = false;
    }

    /// Signals the client that more information must be provided and that the parsing is not finished yet.
    fn exit_but_not_done_yet(&self) -> Result<Option<Frame>, DecodingError> {
        Ok(None)
    }

    fn exit_with_error(&mut self, error: DecodingError) -> Result<Option<Frame>, DecodingError> {
        self.reset();
        Err(error)
    }

    fn exit_with_success(&mut self, frame: Frame) -> Result<Option<Frame>, DecodingError> {
        self.reset();
        Ok(Some(frame))
    }

    fn save_data(&mut self, data: u8) -> Result<Option<Frame>, DecodingError> {
        let payload_reference = self
            .payload_buffer
            .get_mut(self.payload_index)
            .ok_or(DecodingError::BufferOverFlow)?;
        *payload_reference = data;
        self.payload_index += 1;
        self.exit_but_not_done_yet()
    }

    /// Parses asynchronously each byte according to cmpp protocol v1.
    ///
    /// Returns Ok(None) if still decoding, Ok(Some(frame)) if a frame has been parsed and Err if some decidubg error hapenned.
    pub fn parse_next(&mut self, byte: u8) -> Result<Option<Frame>, DecodingError> {
        match self.state {
            State::WaitingFirstEsc => {
                self.state = State::WaitingStartByte;
                Ok(None)
            }

            State::WaitingStartByte => {
                self.start_byte = match byte {
                    STX => Ok(StartByte::STX),
                    ACK => Ok(StartByte::ACK),
                    NACK => Ok(StartByte::NACK),
                    _ => Err(DecodingError::InvalidStartByte(byte)),
                }?;
                self.state = State::ReceivingData;
                self.exit_but_not_done_yet()
            }

            State::ReceivingData => {
                if self.last_was_esc {
                    if byte == ESC {
                        //escdup
                        self.last_was_esc = false;
                        self.save_data(ESC)
                    } else if byte == ETX {
                        //etx
                        self.last_was_esc = false;
                        self.state = State::WaitingChecksum;
                        self.exit_but_not_done_yet()
                    } else {
                        let err = DecodingError::ExpectedEtxOrEscDupButFoundOtherThing(byte);
                        self.exit_with_error(err)
                    }
                } else {
                    if byte == ESC {
                        self.last_was_esc = true;
                        self.exit_but_not_done_yet()
                    } else {
                        //normal data
                        self.save_data(byte)
                    }
                }
            }

            State::WaitingChecksum => {
                /// SAFETY: This function and all its dependent functions are called from inside the State::WaitingChecksum
                fn get_frame(decoder: &Decoder) -> Frame {
                    /// For safety asserts current state is safe to try get frame
                    if decoder.state == State::WaitingChecksum {
                        let frame = Frame {
                            start_byte: decoder.start_byte,
                            payload: decoder.payload_buffer.into(),
                        };
                        Some(frame)
                    } else {
                        None
                    }
                    .unwrap_or_else(|| {
                        // NOTE: This is considered unreachable code because its condition never happens
                        // Error: "Cannot get frame because current state is not "WaitingChecksum"
                        unreachable!("E231");
                    })
                }

                fn validate_checksum(
                    self_: &mut Decoder,
                    incomming_checksum: u8,
                ) -> Result<Frame, DecodingError> {
                    let frame = get_frame(self_);
                    let expected_checksum = frame.checksum();
                    if incomming_checksum == expected_checksum {
                        Ok(frame)
                    } else {
                        Err(DecodingError::InvalidChecksum {
                            expected: expected_checksum,
                            received: expected_checksum,
                        })
                    }
                }

                if self.last_was_esc {
                    if byte == ESC {
                        //Escdup
                        self.last_was_esc = false;
                        let checksum = ESC;
                        let frame = validate_checksum(self, checksum)?;
                        self.exit_with_success(frame)
                    } else {
                        self.exit_with_error(DecodingError::ChecksumIsEscButNotDuplicated(byte))
                    }
                } else {
                    if byte == ESC {
                        self.last_was_esc = true;
                        // check if it is expected that the already received frame should be ESC,
                        // if not early return with an error, else confirm if we will received
                        // an ESC_DUP
                        let frame = get_frame(&self);
                        let expected_checksum = frame.checksum();
                        if expected_checksum == ESC {
                            self.exit_but_not_done_yet()
                        } else {
                            /// We received ESC as checksum but checksum cannot be ESC in this case
                            self.exit_with_error(DecodingError::InvalidChecksum {
                                expected: expected_checksum,
                                received: ESC,
                            })
                        }
                    } else {
                        // non-esc checksum
                        let checksum = byte;
                        let frame = validate_checksum(self, checksum)?;
                        self.exit_with_success(frame)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::protocol::datalink::frame::Payload;

    use super::*;

    fn run_decoder(input: &[u8]) -> Result<Frame, DecodingError> {
        let mut decoder = Decoder::new();
        for byte in input {
            if let Some(frame) = decoder.parse_next(*byte)? {
                return Ok(frame);
            }
        }
        panic!("Input is fully proccessed but no frame result was generated")
    }

    #[test]
    fn it_parse_a_frame() {
        // 1B 02 C1 50 61 02 1B 03 87
        let start_byte = StartByte::STX;
        let start_byte_ = start_byte as u8;
        let probe = [0x1B, start_byte_, 0xC1, 0x50, 0x61, 0x02, 0x1B, 0x03, 0x87];
        let expected = Frame {
            start_byte,
            payload: [0xC1, 0x50, 0x61, 0x02].into(),
        };
        let actual = run_decoder(&probe).unwrap();
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
            payload: [0x01, 0x86, 0x03, 0x1B].into(),
        };
        let actual = run_decoder(&probe).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_parse_a_segment_with_esc_dup_in_checksum_position() {
        // 1B 06 01 86 03 1B 1B 03 52
        let start_byte = StartByte::ACK;
        let start_byte_ = start_byte as u8;
        let probe = [
            0x1B,
            start_byte_,
            0x00,
            0x00,
            0x00,
            0x00 + (247 - 0x1B),
            0x1B,
            0x03,
            0x1B,
            0x1B,
        ];
        let expected = Frame {
            start_byte,
            payload: [0x00, 0x00, 0x00, 0x00 + (247 - 0x1B)].into(),
        };
        let actual = run_decoder(&probe).unwrap();
        assert_eq!(expected, actual);
    }
}
