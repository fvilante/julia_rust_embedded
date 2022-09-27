use super::{common::ETX};
use super::frame::Frame;

// NOTE:
//
//      This checksum calculation is agnostic of the data length. It can calculate checksum
//      for any arbitrary length of data, and not only just cmpp payloads.
//      It depends only on a "[StartbyteNum, obj, ETX]" protocol
//

// NOTE: frame payload should not contain duplicated ESCs
pub fn calc_checksum<const SIZE: usize>(frame: Frame<SIZE>) -> u8 {
    let Frame{start_byte, payload} = frame;
    payload
        .iter()
        .fold(0, |sum, a| sum+a)
        .wrapping_add(start_byte as u8)
        .wrapping_add(ETX)
        .wrapping_neg()
}

#[cfg(test)]
mod tests {
    use crate::protocol::common::StartByte;

    use super::*;

    #[test]
    fn it_calc_checksum_once() {
        // 1B 02 C1 50 61 02 1B 03 87
        let data = [0xC1,0x50,0x61,0x02];
        let frame = Frame{ 
            start_byte: StartByte::STX, 
            payload: data
        };
        let expected = 0x87;
        let result = calc_checksum(frame);
        assert_eq!(expected, result);
    }

    #[test]
    fn it_scan_checksum_range() {
        fn make_package(i: u8) -> [u8;4]{
            //expects: 
            //  - checksum = 0 if i = 0
            //  - higher i, higher checksum; 1 to 1 relationship
            [0xC1,0x50,0x61,0x02+0x87-i] 
        }
        for i in 0..255 as u8 {
            let result = calc_checksum(Frame { 
                start_byte: StartByte::STX, 
                payload: make_package(i)
            });
            assert_eq!(i, result);
        }
    }
}
