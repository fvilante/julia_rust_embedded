use crate::protocol::{frame::Frame, common::StartByte};

use super::word_16::{Word16, BitMask16};

use super::channel::Channel;

type WordAddress = u8;


#[repr(u8)]
enum Direction {
    GetWord = 0x00,
    SetWord = 0xC0,
    SetBitmask = 0x80,
    ResetBitmask = 0x40,
}


pub enum MasterPacket {
    GetWord{
        channel: Channel,
        waddr: WordAddress
    },
    SetWord{
        channel: Channel,
        waddr: WordAddress,
        data: Word16,
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


fn make_payload(message: MasterPacket) -> [u8; 4] {
    match message {
        MasterPacket::GetWord { channel, waddr } => {
            [channel+Direction::GetWord as u8, waddr, 0x00, 0x00]
        }

        MasterPacket::SetWord { channel, waddr, data } => {
            let Word16 { data_high, data_low } = data;
            [channel+Direction::SetWord as u8, waddr, data_low, data_high]
        }

        MasterPacket::ResetBitmask { channel, waddr, bitmask } => {
            let Word16 { data_high, data_low } =Word16::from_bitmask(bitmask);
            [channel+Direction::ResetBitmask as u8, waddr, data_low, data_high]
        }

        MasterPacket::SetBitmask { channel, waddr, bitmask } => {
            let Word16 { data_high, data_low } = Word16::from_bitmask(bitmask);
            [channel+Direction::SetBitmask as u8, waddr, data_low, data_high]
        }
    }
}

pub fn make_frame(message: MasterPacket) -> Frame<4> {
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
        let frame = make_frame(MasterPacket::GetWord { channel, waddr });
        assert_eq!(expected, frame);
    }
}
