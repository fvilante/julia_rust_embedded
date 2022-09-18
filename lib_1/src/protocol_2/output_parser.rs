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

enum StartByte {
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
    checksum: u8,
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
            checksum: 0x00,
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
                self.checksum += start_byte;
                self.state = State::WaitingData0;
                Some(start_byte)
            }

            State::WaitingData0 => {
                let byte = self.read_buffer(0);
                self.checksum += byte;
                self.duplicate_esc_if_necessary(byte, State::WaitingData1)
            }

            State::WaitingData1 => {
                let byte = self.read_buffer(1);
                self.checksum += byte;
                self.duplicate_esc_if_necessary(byte, State::WaitingData2)
            }

            State::WaitingData2 => {
                let byte = self.read_buffer(2);
                self.checksum += byte;
                self.duplicate_esc_if_necessary(byte, State::WaitingData3)
            }

            State::WaitingData3 => {
                let byte = self.read_buffer(3);
                self.checksum += byte;
                self.duplicate_esc_if_necessary(byte, State::WaitingFinalEsc)
            }
            
            State::WaitingFinalEsc => {
                self.state = State::WaitingEtx;
                Some(ESC)
            }

            State::WaitingEtx => {
                self.checksum += ETX;
                self.state = State::WaitingChecksum;
                Some(ETX)
            }

            State::WaitingChecksum => {
                let checksum: u8 = self.checksum;
                // two's compliment of checksum
                let checksum__ = checksum.wrapping_neg()+1;
                self.state = State::Finish;
                self.duplicate_esc_if_necessary(checksum__, State::Finish)
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
    fn it_can_completely_parse_a_simple_frame() {
        let frame = Frame(0xC1, 0x50, 0x61, 0x03);
        let mut encoder = Encoder::new(StartByte::STX, frame);
        let expected = [ESC, STX, 0xC1, 0x50, 0x61, 0x03, ESC, ETX, 0x87];
        let mut buffer = [0x00; 9];
        for index in 0..buffer.len() {
            if let Some(byte) = encoder.get_next() {
                buffer[index] = byte;
            }
        }
        //for index in 0..buffer.len() {
        //    assert_eq!(expected[index], buffer[index]);
        //}
        assert_eq!(expected, buffer);
    }
}



