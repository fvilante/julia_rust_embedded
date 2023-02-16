use super::checksum::calc_checksum;
use super::frame::Frame;

use super::prelude::{ESC, /*STX, ACK, NACK,*/ ETX};

pub enum State {
    WaitingFirstEsc,
    WaitingStartByte,
    WaitingData0,
    WaitingData1,
    WaitingData2,
    WaitingData3,
    WaitingFinalEsc,
    WaitingEtx,
    WaitingChecksum,
    Finish,
}

/// Given a Cmpp Frame, it encodes asynchronously it using the v1.00 Protocol
pub struct Encoder {
    frame: Frame,
    state: State,
    buffer_index: usize,
    last_was_esc: bool,
}

impl Encoder {
    pub fn new(frame: Frame) -> Self {
        Self {
            frame,
            state: State::WaitingFirstEsc,
            buffer_index: 0,
            last_was_esc: false,
        }
    }

    fn duplicate_esc_if_necessary(&mut self, byte: u8, next_state: State) -> Option<u8> {
        if self.last_was_esc {
            self.last_was_esc = false;
            self.state = next_state;
            Some(ESC)
        } else {
            if byte == ESC {
                self.last_was_esc = true;
                Some(ESC)
            } else {
                self.state = next_state;
                Some(byte)
            }
        }
    }

    pub fn get_next(&mut self) -> Option<u8> {
        // none represents end of stream
        match self.state {
            State::WaitingFirstEsc => {
                self.state = State::WaitingStartByte;
                Some(ESC)
            }

            State::WaitingStartByte => {
                let start_byte = self.frame.start_byte as u8;
                self.state = State::WaitingData0;
                Some(start_byte)
            }

            State::WaitingData0 => {
                let byte = self.frame.payload.as_array()[0];
                self.duplicate_esc_if_necessary(byte, State::WaitingData1)
            }

            State::WaitingData1 => {
                let byte = self.frame.payload.as_array()[1];
                self.duplicate_esc_if_necessary(byte, State::WaitingData2)
            }

            State::WaitingData2 => {
                let byte = self.frame.payload.as_array()[2];
                self.duplicate_esc_if_necessary(byte, State::WaitingData3)
            }

            State::WaitingData3 => {
                let byte = self.frame.payload.as_array()[3];
                self.duplicate_esc_if_necessary(byte, State::WaitingFinalEsc)
            }

            State::WaitingFinalEsc => {
                self.state = State::WaitingEtx;
                Some(ESC)
            }

            State::WaitingEtx => {
                self.state = State::WaitingChecksum;
                Some(ETX)
            }

            State::WaitingChecksum => {
                let checksum = calc_checksum(self.frame);
                self.duplicate_esc_if_necessary(checksum, State::Finish)
            }

            State::Finish => None,
        }
    }
}

impl Iterator for Encoder {
    type Item = u8;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        self.get_next()
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::datalink::prelude::StartByte;

    use super::*;

    #[test]
    fn it_can_parse_a_simple_frame_without_esc_dup() {
        // 1B 02 C1 50 61 02 1B 03 87
        let frame = Frame {
            start_byte: StartByte::STX,
            payload: [0xC1, 0x50, 0x61, 0x02].into(),
        };
        let mut encoder = Encoder::new(frame);
        let expected = [0x1B, 0x02, 0xC1, 0x50, 0x61, 0x02, 0x1B, 0x03, 0x87];
        let buffer: [u8; 9] = [0x00; 9];
        let actual = buffer.map(|_| encoder.next().unwrap());
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_can_parse_a_simple_frame_with_esc_dup() {
        // 1B 06 01 86 03 1B 1B 03 52
        let frame = Frame {
            start_byte: StartByte::ACK,
            payload: [0x01, 0x86, 0x03, 0x1B].into(),
        };
        let mut encoder = Encoder::new(frame);
        let expected = [0x1B, 0x06, 0x01, 0x86, 0x03, 0x1B, 0x1B, 0x1B, 0x03, 0x52];
        let buffer = [0x00; 10];
        let actual = buffer.map(|_| encoder.next().unwrap());
        assert_eq!(expected, actual);
    }
}
