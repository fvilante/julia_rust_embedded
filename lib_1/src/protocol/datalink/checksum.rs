use super::frame::Frame;
use super::prelude::ETX;

/// Calculates checksum of the given [`Frame`].
///
/// NOTE: frame payload should not contain duplicated ESCs
pub fn calc_checksum(frame: Frame) -> u8 {
    frame
        .payload
        .as_array()
        .iter()
        .fold(0, |sum, a| sum + a)
        .wrapping_add(frame.start_byte as u8)
        .wrapping_add(ETX)
        .wrapping_neg()
}

#[cfg(test)]
mod tests {
    use crate::protocol::datalink::{frame::Payload, prelude::StartByte};

    use super::*;

    #[test]
    fn it_calc_checksum_once() {
        // 1B 02 C1 50 61 02 1B 03 87
        let data = [0xC1, 0x50, 0x61, 0x02].into();
        let frame = Frame {
            start_byte: StartByte::STX,
            payload: data,
        };
        let expected = 0x87;
        let result = calc_checksum(frame);
        assert_eq!(expected, result);
    }

    #[test]
    fn it_scan_checksum_range() {
        fn make_package(i: u8) -> Payload {
            //expects:
            //  - checksum = 0 if i = 0
            //  - higher i, higher checksum; 1 to 1 relationship
            [0xC1, 0x50, 0x61, 0x02 + 0x87 - i].into()
        }
        for i in 0..255 as u8 {
            let result = calc_checksum(Frame {
                start_byte: StartByte::STX,
                payload: make_package(i),
            });
            assert_eq!(i, result);
        }
    }
}
