use super::checksum::calc_checksum;
#[allow(unused_imports)]


use super::common:: {
    Frame,
    ESC, STX, ACK, NACK, ETX,
};

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

#[derive(Copy, Clone)]
pub enum StartByte {
    STX = 0x02,
    ACK = 0x06,
    NACK = 0x15,
}

struct Encoder {
    start_byte: StartByte,
    frame: Frame,
    state: State, 
    buffer_index: usize,
    last_was_esc: bool,
}

const MAX_PAYLOAD_SIZE: usize = 4;

impl Encoder {
    
    pub fn new(start_byte: StartByte, frame: Frame) -> Self {
        Self {
            start_byte,
            frame,
            state: State::WaitingFirstEsc,
            buffer_index: 0,
            last_was_esc: false,
        }
    }

    fn read_buffer(&self, index: usize) -> u8{
        let Frame(b0,b1,b2,b3) = self.frame;
        match index {
            0 => b0,
            1 => b1,
            2 => b2,
            3 => b3,
            _ => unreachable!(),
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

    pub fn get_next(&mut self) -> Option<u8> {  // none represents end of stream
        match self.state {

            State::WaitingFirstEsc => {
                self.state = State::WaitingStartByte;
                Some(ESC)
            }

            State::WaitingStartByte => {
                let start_byte = self.start_byte as u8;
                self.state = State::WaitingData0;
                Some(start_byte)
            }

            State::WaitingData0 => {
                let byte = self.read_buffer(0);
                self.duplicate_esc_if_necessary(byte, State::WaitingData1)
            }

            State::WaitingData1 => {
                let byte = self.read_buffer(1);
                self.duplicate_esc_if_necessary(byte, State::WaitingData2)
            }

            State::WaitingData2 => {
                let byte = self.read_buffer(2);
                self.duplicate_esc_if_necessary(byte, State::WaitingData3)
            }

            State::WaitingData3 => {
                let byte = self.read_buffer(3);
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
                let Frame(data0, data1, data2, data3) = self.frame;
                let obj = [data0, data1, data2, data3];
                let checksum = calc_checksum(&obj, self.start_byte);
                self.duplicate_esc_if_necessary(checksum, State::Finish)
            }

            State::Finish => {
                None
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

    #[test]
    fn it_can_parse_a_simple_frame_without_esc_dup() {
        // 1B 02 C1 50 61 02 1B 03 87  
        let frame = Frame(0xC1, 0x50, 0x61, 0x02, );
        let mut encoder = Encoder::new(StartByte::STX, frame);
        let expected = [0x1B, 0x02, 0xC1, 0x50, 0x61, 0x02, 0x1B, 0x03, 0x87, ];
        let mut buffer = [0x00; 9];
        for index in 0..buffer.len() {
            if let Some(byte) = encoder.get_next() {
                buffer[index] = byte;
            }
        }
        assert_eq!(expected, buffer);
    }

    #[test]
    fn it_can_parse_a_simple_frame_with_esc_dup() {
        // 1B 06 01 86 03 1B 1B 03 52 
        let frame = Frame(0x01, 0x86, 0x03, 0x1B);
        let mut encoder = Encoder::new(StartByte::ACK, frame);
        let expected = [0x1B, 0x06, 0x01, 0x86, 0x03, 0x1B, 0x1B, 0x1B, 0x03, 0x52 ];
        let mut buffer = [0x00; 10];
        for index in 0..buffer.len() {
            if let Some(byte) = encoder.get_next() {
                buffer[index] = byte;
            }
        }
        assert_eq!(expected, buffer);
    }
}



