pub const ESC: u8 = 0x1B;
pub const STX: u8 = 0x02;
pub const ACK: u8 = 0x06;
pub const NACK: u8 = 0x15;
pub const ETX: u8 = 0x03;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StartByte {
    STX = 0x02,
    ACK = 0x06,
    NACK = 0x15,
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
