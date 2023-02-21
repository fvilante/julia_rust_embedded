use crate::utils::common::{get_bit_at_as_bool, word_to_byte};

//NOTE: In future this type may be extended
pub type BitMask16 = u16;

/// TODO: Make this type obsolete and use the other Word16 instead of this one
pub struct Word16 {
    pub data_high: u8,
    pub data_low: u8,
}

impl Word16 {
    pub fn new(data_low: u8, data_high: u8) -> Self {
        Self {
            data_high,
            data_low,
        }
    }

    pub fn from_u16(data: u16) -> Self {
        let (data_low, data_high) = word_to_byte(data);
        Self {
            data_low,
            data_high,
        }
    }

    pub fn from_bitmask(bitmask: BitMask16) -> Self {
        Self::from_u16(bitmask)
    }

    pub fn to_u16(&self) -> u16 {
        let data_high = self.data_high;
        let data_low = self.data_low;
        let result: u16 = ((data_high as u16) * 256_u16) + data_low as u16;
        result
    }

    /// position of bit in a 16 bits number, that means from 0..=15
    pub fn get_bit(&self, position: u8) -> Option<bool> {
        if position < 16 {
            if position < 8 {
                //byte_low
                Some(get_bit_at_as_bool(self.data_low, position))
            } else {
                //byte_high
                let position__ = position - 8; // rotate bits
                Some(get_bit_at_as_bool(self.data_high, position__))
            }
        } else {
            None
        }
    }
}
