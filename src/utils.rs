/// Concatenate two u8's to one u16
pub fn concat(first: u8, second: u8) -> u16 {
    first as u16 + ((second as u16) << 8)
}

/// Whether the sign bit of a byte is set
pub fn sign_bit(byte: u8) -> bool {
    byte & 0b10000000 == 0b10000000
}
