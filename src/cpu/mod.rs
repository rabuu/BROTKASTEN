mod instruction;
mod status_flag;

/// The `MOS6510` 8-bit CPU
#[derive(Debug)]
pub struct MOS6510 {
    /// accumulator register
    a: u8,
    /// index register
    x: u8,
    /// index register
    y: u8,
    /// stack pointer
    s: u8,
    /// program counter
    pc: u16,
    /// processor status flag
    p: u8,

    /// 64kB RAM
    ram: Box<[u8; 64_000]>,
}
