/// Bitwise operations

/// NOTE: Deprecated! Use "get_bit_at_as_bool" instead.
/// position 0 = bit0, position 1 = bit1, ...
pub const fn get_bit_at(byte: u8, position: u8) -> u8 {
    byte & (1 << position)
}

/// position 0 = bit0, position 1 = bit1, ...
pub const fn get_bit_at_as_bool(byte: u8, position: u8) -> bool {
    let bit = byte & (1 << position);
    if bit == 0 {
        false
    } else {
        true
    }
}

/// position 0 = bit0, position 1 = bit1, ...
pub const fn set_bit_at(byte: u8, position: u8) -> u8 {
    byte | (1 << position)
}

pub const fn invert(byte: u8) -> u8 {
    !byte
}

/// position 0 = bit0, position 1 = bit1, ...
pub const fn reset_bit_at(byte: u8, position: u8) -> u8 {
    !set_bit_at(!byte, position)
}

pub const fn configure_bit(byte: u8, position: u8, data_bit: bool) -> u8 {
    match data_bit {
        true => set_bit_at(byte, position),
        false => reset_bit_at(byte, position),
    }
}
