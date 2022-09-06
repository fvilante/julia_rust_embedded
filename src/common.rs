

// return (byteLow, byteHigh)
pub fn word_to_byte(word: u16) -> (u8, u8) {
    let low_ = word & 0x00FF;
    let byte_low  = low_ as u8;
    let high_ = (word >> 8) & 0x00FF;
    let byte_high = high_ as u8;
    (byte_low, byte_high)
}

// position 0 = bit0, position 1 = bit1, ...
pub fn get_bit_at(data: u8, position: u8) -> u8 {
    data & (1 << position)
}
