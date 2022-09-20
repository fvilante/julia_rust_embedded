pub const ESC: u8 = 0x1B;
pub const STX: u8 = 0x02;
pub const ACK: u8 = 0x06;
pub const NACK: u8 = 0x15;
pub const ETX: u8 = 0x03;


#[derive(Debug,PartialEq)]
pub struct Frame(pub u8, pub u8, pub u8, pub u8);

impl Frame {
    pub fn to_array(self) -> [u8;4] {
        let Frame(d0, d1, d2, d3) = self;
        let arr = [d0,d1,d2,d3];
        return arr
    }

    pub fn from_array(arr: &[u8;4]) -> Frame {
        let [ d0, d1, d2, d3 ] = arr;
        Frame(d0.clone(), d1.clone(), d2.clone(), d3.clone())
    }

}


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum StartByte {
    STX = 0x02,
    ACK = 0x06,
    NACK = 0x15,
}

