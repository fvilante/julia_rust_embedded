use crate::protocol::datalink::datalink::word16::{self, Word16};
use crate::protocol::datalink::frame::Payload;
use crate::protocol::datalink::{frame::Frame, prelude::StartByte};

use super::channel::Channel;
use super::transport_error::TransportError;

type WordAddress = u8;

#[repr(u8)]
enum Direction {
    GetWord = 0x00,
    SetWord = 0xC0,
    SetBitmask = 0x80,
    ResetBitmask = 0x40,
}

pub enum CmppMessage {
    GetWord { waddr: WordAddress },
    SetWord { waddr: WordAddress, data: Word16 },
    SetBitmask { waddr: WordAddress, bitmask: u16 },
    ResetBitmask { waddr: WordAddress, bitmask: u16 },
}

fn make_payload(channel: Channel, message: CmppMessage) -> Payload {
    let [direction, waddr, byte_low, byte_high] = match message {
        CmppMessage::GetWord { waddr } => [Direction::GetWord as u8, waddr, 0x00, 0x00],

        CmppMessage::SetWord { waddr, data } => {
            let (data_high, data_low) = data.split_bytes();
            [Direction::SetWord as u8, waddr, data_low, data_high]
        }

        CmppMessage::ResetBitmask { waddr, bitmask } => {
            let (data_high, data_low) = Word16::from_u16(bitmask).split_bytes();
            [Direction::ResetBitmask as u8, waddr, data_low, data_high]
        }

        CmppMessage::SetBitmask { waddr, bitmask } => {
            let (data_high, data_low) = Word16::from_u16(bitmask).split_bytes();
            [Direction::SetBitmask as u8, waddr, data_low, data_high]
        }
    };

    [channel.to_u8() + direction, waddr, byte_low, byte_high].into()
}

pub fn make_frame(channel: Channel, message: CmppMessage) -> Frame {
    let start_byte = StartByte::STX;
    let payload = make_payload(channel, message);
    let frame = Frame {
        start_byte,
        payload,
    };
    frame
}

#[cfg(test)]
mod tests {

    use crate::protocol::transport::channel::Channel;

    use super::*;

    #[test]
    fn it_create_get_word_frame() {
        let channel = Channel::from_u8(0x01).unwrap();
        let waddr = 0x50;
        let expected = Frame {
            start_byte: StartByte::STX,
            payload: [
                channel.to_u8() + Direction::GetWord as u8,
                waddr,
                0x00,
                0x00,
            ]
            .into(),
        };
        let frame = make_frame(channel, CmppMessage::GetWord { waddr });
        assert_eq!(expected, frame);
    }
}
