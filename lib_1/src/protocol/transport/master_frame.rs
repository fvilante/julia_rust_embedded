use crate::protocol::{frame::Frame, common::StartByte};

type Channel = u8;
type WordAddress = u8;
type Word16 = u16;
type BitMask16 = u16;

#[repr(u8)]
enum Direction {
    GetWord = 0x00,
    SetWord = 0xC0,
    SetBitmask = 0x80,
    ResetBitmask = 0x40,
}


pub enum MasterFrame {
    GetWord{
        channel: Channel,
        waddr: WordAddress
    },
    SetWord{
        channel: Channel,
        waddr: WordAddress,
        data: BitMask16,
    },
    SetBitmask{
        channel: Channel,
        waddr: WordAddress,
        bitmask: BitMask16,
    },
    ResetBitmask{
        channel: Channel,
        waddr: WordAddress,
        bitmask: BitMask16,
    },
}


fn make_payload(message: MasterFrame) -> [u8; 4] {
    match message {
        MasterFrame::GetWord { channel, waddr } => {
            [channel+Direction::GetWord as u8, waddr, 0x00, 0x00]
        }

        MasterFrame::SetWord { channel, waddr, data } => {
            [channel+Direction::SetWord as u8, waddr, 0x00, 0x00]
        }

        MasterFrame::ResetBitmask { channel, waddr, bitmask } => {
            [channel+Direction::ResetBitmask as u8, waddr, 0x00, 0x00]
        }

        MasterFrame::SetBitmask { channel, waddr, bitmask } => {
            [channel+Direction::SetBitmask as u8, waddr, 0x00, 0x00]
        }
    }
}

pub fn make_frame(message: MasterFrame) -> Frame<4> {
    let start_byte = StartByte::STX;
    let payload: [u8;4] = make_payload(message);
    Frame{start_byte, payload}
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_create_get_word_frame() {
        let channel = 0x01;
        let waddr = 0x50;
        let expected = Frame{
            start_byte: StartByte::STX,
            payload: [channel+Direction::GetWord as u8, waddr, 0x00, 0x00],
        };
        let frame = make_frame(MasterFrame::GetWord { channel, waddr });
        assert_eq!(expected, frame);
    }
}
