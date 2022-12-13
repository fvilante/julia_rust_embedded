use core::{str::FromStr, marker::Destruct};
use heapless::String;



// return (byteLow, byteHigh)
pub fn word_to_byte(word: u16) -> (u8, u8) {
    let low_ = word & 0x00FF;
    let byte_low  = low_ as u8;
    let high_ = (word >> 8) & 0x00FF;
    let byte_high = high_ as u8;
    (byte_low, byte_high)
}

// NOTE: Deprecated! Use "get_bit_at_as_bool" instead.
// position 0 = bit0, position 1 = bit1, ...
pub fn get_bit_at(byte: u8, position: u8) -> u8 {
    byte & (1 << position)
}

// position 0 = bit0, position 1 = bit1, ...
pub fn get_bit_at_as_bool(byte: u8, position: u8) -> bool {
    let bit = byte & (1 << position);
    if bit == 0 {
        false
    } else {
        true
    }
}


// position 0 = bit0, position 1 = bit1, ...
pub fn set_bit_at(byte: u8, position: u8) -> u8 {
    byte | (1 << position)
}

pub fn invert(byte: u8) -> u8 {
    !byte
}

// position 0 = bit0, position 1 = bit1, ...
pub fn reset_bit_at(byte: u8, position: u8) -> u8 {
    !set_bit_at(!byte, position)
}

pub fn configure_bit(byte: u8, position: u8, data_bit: bool) -> u8 {
    match data_bit {
        true => set_bit_at(byte, position),
        false => reset_bit_at(byte, position),
    }
}


const TABLE: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E', 'F'];

// NOTE: return folows the order: "Most Significant on Right"
pub fn convert_u8_to_str_hex(data: u8) -> (char,char) {

    let low: u8 = data & 0b00001111;
    let high: u8 = (data & 0b11110000) >> 4;

    let low_char = TABLE[low as usize];
    let high_char = TABLE[high as usize];

    (high_char, low_char)
}

// NOTE: return folows the order: "Most Significant on Right"
pub fn convert_u16_to_str_hex(data: u16) -> (char,char,char,char) {
    let low: u8 = (data & 0x00FF) as u8;
    let high: u8 = ((data & 0xFF00) >> 8) as u8;

    let (lowbyte_high, lowbyte_low) = convert_u8_to_str_hex(low);
    let (highbyte_high, highbyte_low) = convert_u8_to_str_hex(high);

    (highbyte_high, highbyte_low, lowbyte_high, lowbyte_low) 
}

//NOTE: value above 65535 are clamped
pub fn convert_u16_to_string_decimal (value: u16) -> String<5> {

    let mut buffer = itoa::Buffer::new();
    let printed = buffer.format(value as u64);
    let string: String<5> = String::from_str(printed).unwrap();
    string
}

/// Equals std::clamp but with constant function attribute 
pub const fn const_clamp<T>(data: T, min: T, max: T) -> T
where
    T: Sized,
    T: ~const Destruct,
    T: ~const PartialOrd,
{
    assert!(min <= max);
    if data < min {
        min
    } else if data > max {
        max
    } else {
        data
    }
}

pub const fn usize_to_u8_clamper(data: usize) -> u8 {
    let clamped_u8 = const_clamp(data, u8::MIN as usize, u8::MAX as usize) as u8;
    clamped_u8
}

pub const fn u32_to_u16_clamper(data: u32) -> u16 {
    let clamped_u8 = const_clamp(data, u16::MIN as u32, u16::MAX as u32) as u16;
    clamped_u8
}
