use super::{encoder::Encoder, prelude::StartByte};

pub type Payload = [u8; 4]; // [ Direcao+canal; Cmd; dada_low, data_high]

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Frame {
    pub start_byte: StartByte,
    pub payload: Payload,
}

impl Frame {
    pub const fn new(start_byte: StartByte, payload: Payload) -> Self {
        Self {
            start_byte,
            payload,
        }
    }

    pub const fn make_master_block(payload: Payload) -> Self {
        Self::new(StartByte::STX, payload)
    }

    pub fn encode(&self) -> Encoder {
        Encoder::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_frame_works() {
        //prepare
        let expected: [u8; 4] = [1, 2, 3, 4];
        let start_byte = StartByte::STX;
        //act
        let frame = Frame::new(start_byte, expected);
        //check
        assert_eq!(expected, frame.payload);
        assert_eq!(start_byte, frame.start_byte);
    }
}
