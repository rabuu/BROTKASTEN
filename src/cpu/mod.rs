use crate::memory::Ram;

use self::instruction::addressing::AddrOperand;
use self::instruction::Operation;

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

    /// memory
    ram: Ram,
}

impl MOS6510 {
    pub fn fetch_decode(&mut self) -> Option<Operation> {
        let op_byte: u8 = self.ram.read(self.pc).unwrap();

        let (opcode, addr_mode) = instruction::INSTRUCTIONS[op_byte as usize]?;
        let operand: AddrOperand = addr_mode.get_operand(self);
        let operation: Operation = (opcode, operand);

        self.pc = self.pc.wrapping_add(1 + addr_mode.addr_size());

        Some(operation)
    }
}
