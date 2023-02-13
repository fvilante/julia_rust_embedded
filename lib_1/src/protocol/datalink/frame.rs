use super::{
    encoder::Encoder,
    prelude::{MasterStartByte, SlaveStartByte, StartByte},
};

pub type Payload = [u8; 4]; // [ Direcao+canal; Cmd; dada_low, data_high]

//////////////////////////////////////////////////

/// Represents a frame created by Master (That means start byte is necessaryly STX)
pub struct MasterFrame {
    pub start_byte: MasterStartByte,
    pub payload: Payload,
}

/// Fails if frame.start_byte is NOT equals to STX
impl TryFrom<Frame> for MasterFrame {
    type Error = ();

    fn try_from(frame: Frame) -> Result<Self, Self::Error> {
        if let Ok(start_byte) = frame.start_byte.try_into() {
            Ok(MasterFrame {
                start_byte,
                payload: frame.payload,
            })
        } else {
            Err(())
        }
    }
}

/// Represents a frame created by Slave (That means start byte MUST NOT be STX)
pub struct SlaveFrame {
    pub start_byte: SlaveStartByte,
    pub payload: Payload,
}

/// Fails if frame.start_byte IS equals to STX
impl TryFrom<Frame> for SlaveFrame {
    type Error = ();

    fn try_from(frame: Frame) -> Result<Self, Self::Error> {
        frame.to_slave_frame()
    }
}

//////////////////////////////////////////////////

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

    pub fn to_slave_frame(&self) -> Result<SlaveFrame, ()> {
        if let Ok(start_byte) = self.start_byte.try_into() {
            Ok(SlaveFrame {
                start_byte,
                payload: self.payload,
            })
        } else {
            Err(())
        }
    }

    /// TODO: Deprecate, prefer to convert to [`MasterFrame`] type
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
