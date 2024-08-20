use flags::Flags;
use instruction::opcode::Opcode;

use crate::memory::Memory;

use self::instruction::{Operand, Operation};

mod flags;
mod instruction;

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
    pub p: Flags,

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

    fn fetch_decode(&mut self) -> Option<Operation> {
        let op_byte: u8 = self.mem.read(self.pc);

        let (opcode, addr_mode) = instruction::INSTRUCTIONS[op_byte as usize]?;
        let operand: Operand = addr_mode.get_operand(self);
        let operation: Operation = (opcode, operand);

        self.pc = self.pc.wrapping_add(1 + addr_mode.addr_size());

        Some(operation)
    }

    fn load_acc(&mut self, val: u8) {
        self.acc = val;
        self.p.overwrite(Flags::Z, val == 0);
        self.p.overwrite(Flags::N, val & 0b10000000 != 0);
    }

    fn execute_operation(&mut self, op: Operation) {
        match op {
            (Opcode::LDA, Operand::Address(addr)) => {
                let val = self.mem.read(addr);
                tracing::debug!("load {val} (${addr}) to acc");
                self.load_acc(val);
            }
            (Opcode::SEC, Operand::Implied) => {
                tracing::debug!("set carry");
                self.p |= Flags::C;
            }
            (Opcode::BEQ, Operand::AddressOffset(offset)) => {
                tracing::debug!("branch if equal (zero set) -> {offset}");
                if self.p.contains(Flags::Z) {
                    tracing::debug!("branch taken");
                    self.pc = self.pc.wrapping_add(offset);
                }
            }
            (Opcode::BMI, Operand::AddressOffset(offset)) => {
                tracing::debug!("branch on minus (negative set) -> {offset}");
                if self.p.contains(Flags::N) {
                    tracing::debug!("branch taken");
                    self.pc = self.pc.wrapping_add(offset);
                }
            }
            (Opcode::STA, Operand::Address(addr)) => {
                tracing::debug!("store acc to memory at ${addr}");
                self.mem.write(addr, self.acc);
            }
            (Opcode::JMP, Operand::Address(addr)) => {
                tracing::debug!("jump to ${addr}");
                self.pc = addr;
            }
            (opcode, operand) => {
                tracing::error!(
                    "Unimplemented opcode {:?} for operand {:?}",
                    opcode,
                    operand
                );
                std::process::exit(1);
            }
        }
    }

    pub fn run(&mut self) {
        while let Some(op) = self.fetch_decode() {
            self.execute_operation(op);
        }
    }
}
