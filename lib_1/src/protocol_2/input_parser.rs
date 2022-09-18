use super::common::*;

const MAX_BUFFER_LEN: usize = 4; // max data length buffer

struct InputParser {
    state: State,
    buffer_index: usize,
    buffer: [u8;MAX_BUFFER_LEN],
    last_was_esc: bool,
}

impl InputParser {

    pub fn new() -> Self {
        Self {
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

    fn success(&self, _checksum: u8) -> Frame {
        //Fix: consider checksum in calc
        let b0 = self.buffer[0];
        let b1 = self.buffer[1];
        let b2 = self.buffer[2];
        let b3 = self.buffer[3];
        Frame(b0,b1,b2,b3)
    }

    pub fn parse_next(&mut self, byte: u8) -> Result<Option<Frame>, SegmentError> {
        match self.state {
            State::WaitingFirstEsc => {
                self.state = State::WaitingStartByte;
                Ok(None)
            }

            State::WaitingStartByte => {
                let is_start_byte = byte == STX || byte == ACK || byte == NACK;
                if is_start_byte {
                    self.state = State::ReceivingData;
                    Ok(None)
                } else {
                    Err(SegmentError::InvalidStartByte(byte))
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
                        Err(SegmentError::ExpectedEtxOrEscDupBufFoundOtherThing(byte))
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
                        Ok(Some(self.success(checksum)))
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
                        let checksum = ESC;
                        Ok(Some(self.success(checksum)))
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

    fn perform_test(input_probe: &[u8], expected: Frame) -> () { 
        let mut parser = InputParser::new();
        let mut success: bool = false;
        for byte in input_probe {
            let result = parser.parse_next(*byte);
            if let Ok(Some(frame)) = result {
                assert_eq!(frame, expected);
                success = true;
            } 
        }
        assert_eq!(success, true);
    }

    #[test]
    fn it_can_parse_a_segment() {
        let input_probe = [ESC,STX,0,1,2,3,ESC,ETX,0];
        let expected = Frame(0,1,2,3);
        perform_test(&input_probe, expected);
    }

    #[test]
    fn it_can_parse_a_segment_with_esc_dup_in_data_position_0() {
        let input_probe = [ESC,STX,27,27,1,2,3,ESC,ETX,0];
        let expected = Frame(27,1,2,3);
        perform_test(&input_probe, expected);
    }

    #[test]
    fn it_can_parse_a_segment_with_esc_dup_in_data_position_1() {
        let input_probe = [ESC,STX,0,27,27,2,3,ESC,ETX,0];
        let expected = Frame(0,27,2,3);
        perform_test(&input_probe, expected);
    }

    #[test]
    fn it_can_parse_a_segment_with_esc_dup_in_data_position_2() {
        let input_probe = [ESC,STX,0,1,27,27,3,ESC,ETX,0];
        let expected = Frame(0,1,27,3);
        perform_test(&input_probe, expected);
    }

    #[test]
    fn it_can_parse_a_segment_with_esc_dup_in_data_position_3() {
        let input_probe = [ESC,STX,0,1,2,27,27,ESC,ETX,0];
        let expected = Frame(0,1,2,27);
        perform_test(&input_probe, expected);
    }

    #[test]
    fn it_can_parse_a_segment_with_esc_dup_in_checksum_position() {
        let input_probe = [ESC,STX,0,1,2,3,ESC,ETX,27,27];
        let expected = Frame(0,1,2,3);
        perform_test(&input_probe, expected);
    }



}
