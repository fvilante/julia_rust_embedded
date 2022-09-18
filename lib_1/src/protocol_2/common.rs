pub const ESC: u8 = 0x1B;
pub const STX: u8 = 0x02;
pub const ACK: u8 = 0x06;
pub const NACK: u8 = 0x15;
pub const ETX: u8 = 0x03;


pub enum State {
    WaitingFirstEsc,
    WaitingStartByte,
    ReceivingData,
    WaitingChecksum,
}

#[derive(Debug,PartialEq)]
pub struct Frame(pub u8, pub u8, pub u8, pub u8);

pub enum SegmentError {
    InvalidStartByte(u8),
    BufferOverFlow,
    ExpectedEtxOrEscDupBufFoundOtherThing(u8),
    ChecksumIsEscButNotDuplicated(u8),
}