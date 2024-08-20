use crate::memory::Memory;
use crate::utils::*;
use flags::Flags;
use instruction::opcode::Opcode;
use instruction::{Operand, Operation};

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
        self.p.overwrite(Flags::N, sign_bit(val));
    }

    fn load_x(&mut self, val: u8) {
        self.x = val;
        self.p.overwrite(Flags::Z, val == 0);
        self.p.overwrite(Flags::N, sign_bit(val));
    }

    fn load_y(&mut self, val: u8) {
        self.y = val;
        self.p.overwrite(Flags::Z, val == 0);
        self.p.overwrite(Flags::N, sign_bit(val));
    }

    fn execute_operation(&mut self, op: Operation) {
        match op {
            (Opcode::LDA, Operand::Address(addr)) => {
                let val = self.mem.read(addr);
                tracing::debug!("load {val} ({addr:#x}) to acc");
                self.load_acc(val);
            }
            (Opcode::LDX, Operand::Address(addr)) => {
                let val = self.mem.read(addr);
                tracing::debug!("load {val} ({addr:#x}) to X");
                self.load_x(val);
            }
            (Opcode::LDY, Operand::Address(addr)) => {
                let val = self.mem.read(addr);
                tracing::debug!("load {val} ({addr:#x}) to Y");
                self.load_y(val);
            }
            (Opcode::SEC, Operand::Implied) => {
                tracing::debug!("set carry");
                self.p |= Flags::C;
            }
            (Opcode::ADC, Operand::Address(addr)) => {
                let val = self.mem.read(addr);
                tracing::debug!("add {val} ({addr:#x}) with carry");

                // TODO: decimal mode

                let carry = if self.p.contains(Flags::C) { 1 } else { 0 };
                let new_acc = self.acc.wrapping_add(val).wrapping_add(carry);

                // carry
                self.p.overwrite(
                    Flags::C,
                    new_acc < self.acc || (carry == 1 && (new_acc == 0 || val == 0xff)),
                );

                // overflow
                self.p.overwrite(
                    Flags::V,
                    (sign_bit(self.acc) && sign_bit(val) && !sign_bit(new_acc))
                        || (!sign_bit(self.acc) && !sign_bit(val) && sign_bit(new_acc)),
                );

                self.load_acc(new_acc);
            }
            (Opcode::SBC, Operand::Address(addr)) => {
                let val = self.mem.read(addr);
                tracing::debug!("substract {val} ({addr:#x}) with carry");

                // TODO: decimal mode

                let not_carry = if self.p.contains(Flags::C) { 0 } else { 1 };
                let new_acc = self.acc.wrapping_sub(val).wrapping_sub(not_carry);

                // carry (when unsigned overflow occured)
                self.p.overwrite(Flags::C, new_acc > self.acc);

                // overflow
                self.p.overwrite(
                    Flags::V,
                    (not_carry == 0 && !sign_bit(self.acc) && sign_bit(val) && sign_bit(new_acc))
                        || (sign_bit(self.acc)
                            && sign_bit(0u8.wrapping_sub(val).wrapping_sub(not_carry))
                            && !sign_bit(new_acc)),
                );

                self.load_acc(new_acc);
            }
            (Opcode::BEQ, Operand::AddressOffset(offset)) => {
                tracing::debug!("beq (Z set), acc: {}, offset: {offset}", self.acc);
                if self.p.contains(Flags::Z) {
                    tracing::debug!("branch taken");
                    self.pc = self.pc.wrapping_add(offset);
                }
            }
            (Opcode::BMI, Operand::AddressOffset(offset)) => {
                tracing::debug!("bmi (N set), acc: {}, offset: {offset}", self.acc);
                if self.p.contains(Flags::N) {
                    tracing::debug!("branch taken");
                    self.pc = self.pc.wrapping_add(offset);
                }
            }
            (Opcode::STA, Operand::Address(addr)) => {
                tracing::debug!("store acc={} to memory at {addr:#x}", self.acc);
                self.mem.write(addr, self.acc);
            }
            (Opcode::STX, Operand::Address(addr)) => {
                tracing::debug!("store X={} to memory at {addr:#x}", self.x);
                self.mem.write(addr, self.x);
            }
            (Opcode::STY, Operand::Address(addr)) => {
                tracing::debug!("store Y={} to memory at {addr:#x}", self.y);
                self.mem.write(addr, self.y);
            }
            (Opcode::JMP, Operand::Address(addr)) => {
                tracing::debug!("jump to {addr:#x}");
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
