

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
pub fn get_bit_at(data: u8, position: u8) -> u8 {
    data & (1 << position)
}

// position 0 = bit0, position 1 = bit1, ...
pub fn get_bit_at_as_bool(data: u8, position: u8) -> bool {
    let bit = data & (1 << position);
    if bit == 0 {
        false
    } else {
        true
    }
}

const TABLE: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E', 'F'];

// convert u8 to str
pub fn convert_u8_to_str_hex(data: u8) -> (char,char) {

    let low: u8 = data & 0b00001111;
    let high: u8 = (data & 0b11110000) >> 4;

    let low_char = TABLE[low as usize];
    let high_char = TABLE[high as usize];

    (high_char, low_char)
}

