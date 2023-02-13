use crate::protocol::datalink::frame::Payload;
use crate::protocol::datalink::{frame::Frame, prelude::StartByte};

use super::transport_error::TransportError;
use super::word_16::{BitMask16, Word16};

use crate::protocol::transport::channel::Channel;

type WordAddress = u8;

#[repr(u8)]
enum Direction {
    GetWord = 0x00,
    SetWord = 0xC0,
    SetBitmask = 0x80,
    ResetBitmask = 0x40,
}

pub enum CmppMessage {
    GetWord {
        waddr: WordAddress,
    },
    SetWord {
        waddr: WordAddress,
        data: Word16,
    },
    SetBitmask {
        waddr: WordAddress,
        bitmask: BitMask16,
    },
    ResetBitmask {
        waddr: WordAddress,
        bitmask: BitMask16,
    },
}

fn make_payload(channel: Channel, message: CmppMessage) -> Result<Payload, TransportError> {
    let [direction, waddr, byte_low, byte_high] = match message {
        CmppMessage::GetWord { waddr } => [Direction::GetWord as u8, waddr, 0x00, 0x00],

        CmppMessage::SetWord { waddr, data } => {
            let Word16 {
                data_high,
                data_low,
            } = data;
            [Direction::SetWord as u8, waddr, data_low, data_high]
        }

        CmppMessage::ResetBitmask { waddr, bitmask } => {
            let Word16 {
                data_high,
                data_low,
            } = Word16::from_bitmask(bitmask);
            [Direction::ResetBitmask as u8, waddr, data_low, data_high]
        }

        CmppMessage::SetBitmask { waddr, bitmask } => {
            let Word16 {
                data_high,
                data_low,
            } = Word16::from_bitmask(bitmask);
            [Direction::SetBitmask as u8, waddr, data_low, data_high]
        }
    };

    channel
        .as_u8()
        .map(|channel| [channel + direction, waddr, byte_low, byte_high])
        .ok_or_else(|| TransportError::InvalidChannel(channel))
}

pub fn make_frame(channel: Channel, message: CmppMessage) -> Result<Frame, TransportError> {
    let start_byte = StartByte::STX;
    let payload = make_payload(channel, message);
    payload.map(|payload| Frame {
        start_byte,
        payload,
    })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_create_get_word_frame() {
        let channel = Channel::new(0x01);
        let waddr = 0x50;
        let expected = Frame {
            start_byte: StartByte::STX,
            payload: [
                channel.as_u8().unwrap() + Direction::GetWord as u8,
                waddr,
                0x00,
                0x00,
            ],
        };
        let frame = make_frame(channel, CmppMessage::GetWord { waddr });
        assert_eq!(expected, frame.unwrap());
    }
}
