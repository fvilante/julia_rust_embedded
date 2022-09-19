#![allow(unused_imports)]
#![allow(unused_variables)]
use super::{encoder::StartByte, common::ETX};

// NOTE:
//
//      This checksum calculation is agnostic of the data length. It can calculate checksum
//      for any arbitrary length of data, and not only just cmpp payloads.
//      It depends only on a "[StartbyteNum, obj, ETX]" protocol
//

// NOTE: Data should not contain duplicated ESCs
pub fn calc_checksum(obj: &[u8], start_byte: StartByte) -> u8 {
    type Size = u16;
    let mut objsum: Size = 0x00;
    for each_byte in obj {
        objsum = objsum + (each_byte.clone() as Size);
    };
    let extra = (start_byte as Size) + ETX as Size; 
    let totalsum = objsum + extra;
    let contained = totalsum % 256;
    let complimented = 256 - contained;
    let adjusted = if complimented == 256 { 0 } else { complimented };
    // TODO: assure return is in uint8 range
    return adjusted.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calc_checksum() {
        // 1B 02 C1 50 61 02 1B 03 87
        let data = [0xC1,0x50,0x61,0x02];
        let start_byte = StartByte::STX;
        let expected = 0x87;
        let result = calc_checksum(&data, start_byte);
        assert_eq!(expected, result);
    }
}
