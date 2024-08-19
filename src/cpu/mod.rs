use crate::memory::Memory;

use self::instruction::addressing::AddrOperand;
use self::instruction::Operation;

mod instruction;
mod status_flag;

/// The `MOS6510` 8-bit CPU
#[derive(Debug, Default)]
pub struct MOS6510 {
    /// accumulator register
    pub acc: u8,

    /// index register X
    pub x: u8,

    /// index register Y
    pub y: u8,

    /// stack pointer
    pub sp: u8,

    /// program counter
    pub pc: u16,

    /// processor status flag
    pub p: u8,

    /// memory
    pub mem: Memory,
}

impl MOS6510 {
    pub fn new(memory: Memory) -> MOS6510 {
        MOS6510 {
            mem: memory,
            ..Default::default()
        }
    }

    pub fn fetch_decode(&mut self) -> Option<Operation> {
        let op_byte: u8 = self.mem.read(self.pc);

        let (opcode, addr_mode) = instruction::INSTRUCTIONS[op_byte as usize]?;
        let operand: AddrOperand = addr_mode.get_operand(&self);
        let operation: Operation = (opcode, operand);

        self.pc = self.pc.wrapping_add(1 + addr_mode.addr_size());

        Some(operation)
    }

    pub fn run(&mut self) {
        todo!();
    }
}
