pub const ESC: u8 = 0x1B;
pub const STX: u8 = 0x02;
pub const ACK: u8 = 0x06;
pub const NACK: u8 = 0x15;
pub const ETX: u8 = 0x03;

/// Indicates what variations are allowed to the StartByte
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum StartByte {
    STX = STX,
    ACK = ACK,
    NACK = NACK,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum SlaveStartByte {
    ACK = ACK,
    NACK = NACK,
}

impl TryFrom<StartByte> for SlaveStartByte {
    type Error = ();

    fn try_from(start_byte: StartByte) -> Result<Self, Self::Error> {
        match start_byte {
            StartByte::STX => Err(()),
            StartByte::ACK => Ok(SlaveStartByte::ACK),
            StartByte::NACK => Ok(SlaveStartByte::NACK),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum MasterStartByte {
    STX = STX,
}

impl TryFrom<StartByte> for MasterStartByte {
    type Error = ();

    fn try_from(start_byte: StartByte) -> Result<Self, Self::Error> {
        match start_byte {
            StartByte::STX => Ok(MasterStartByte::STX),
            StartByte::ACK => Err(()),
            StartByte::NACK => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_correctly_defined_start_bytes() {
        assert_eq!(StartByte::STX as u8, 0x02);
        assert_eq!(StartByte::ACK as u8, 0x06);
        assert_eq!(StartByte::NACK as u8, 0x15);
    }

    #[test]
    fn it_is_correctly_control_bytes() {
        assert_eq!(ESC, 0x1B);
        assert_eq!(STX, 0x02);
        assert_eq!(ACK, 0x06);
        assert_eq!(NACK, 0x15);
        assert_eq!(ETX, 0x03);
    }
}
