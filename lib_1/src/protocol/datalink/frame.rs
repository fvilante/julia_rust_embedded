use super::{
    checksum::calc_checksum,
    datalink::Word16,
    encoder::Encoder,
    prelude::{MasterStartByte, SlaveStartByte, StartByte},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Payload {
    pub direction_and_channel: u8,
    pub command: u8,
    pub byte_low: u8,
    pub byte_high: u8,
} // [ Direcao+canal; Cmd; dada_low, data_high]

impl Payload {
    pub fn from_array(array: [u8; 4]) -> Self {
        let [direction_and_channel, command, byte_low, byte_high] = array;
        Self {
            direction_and_channel,
            command,
            byte_low,
            byte_high,
        }
    }

    pub fn as_array(&self) -> [u8; 4] {
        [
            self.direction_and_channel,
            self.command,
            self.byte_low,
            self.byte_high,
        ]
    }

    /// Returns (byte_low, byte_high)
    pub fn get_word(&self) -> Word16 {
        Word16::from_bytes(self.byte_low, self.byte_high)
    }
}

impl From<[u8; 4]> for Payload {
    fn from(value: [u8; 4]) -> Self {
        Self::from_array(value)
    }
}

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

#[derive(Clone)]
pub struct SlaveFrameAck(pub SlaveFrame);

#[derive(Clone)]
pub struct SlaveFrameNack(pub SlaveFrame);

/// Represents a frame created by Slave (That means start byte MUST NOT be STX)
#[derive(Clone)]
pub struct SlaveFrame {
    pub start_byte: SlaveStartByte,
    pub payload: Payload,
}

impl SlaveFrame {
    pub fn kind(&self) -> Result<SlaveFrameAck, SlaveFrameNack> {
        match self.start_byte {
            SlaveStartByte::ACK => Ok(SlaveFrameAck(self.clone())),
            SlaveStartByte::NACK => Err(SlaveFrameNack(self.clone())),
        }
    }
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

    pub fn checksum(&self) -> u8 {
        calc_checksum(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_frame_works() {
        //prepare
        let expected = [1, 2, 3, 4].into();
        let start_byte = StartByte::STX;
        //act
        let frame = Frame::new(start_byte, expected);
        //check
        assert_eq!(expected, frame.payload);
        assert_eq!(start_byte, frame.start_byte);
    }
}
