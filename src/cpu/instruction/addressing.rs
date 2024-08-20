use super::Operand;
use crate::cpu::MOS6510;
use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    /// implied
    Imp,
    /// absolute
    Abs,
    /// X-indexed absolute
    Abx,
    /// Y-indexed absolute
    Aby,
    /// zero-page
    Zpg,
    /// X-indexed zero-page
    Zpx,
    /// Y-indexed zero-page
    Zpy,
    /// relative
    Rel,
    /// accumulator
    Akk,
    /// indirect
    Ind,
    /// X-indexed indirect
    Inx,
    /// Y-indexed indirect
    Iny,
    /// immediate
    Imm,
}

impl AddressingMode {
    pub const fn addr_size(&self) -> u16 {
        use AddressingMode::*;
        match self {
            Imp | Akk => 0,
            Zpg | Zpx | Zpy | Rel | Inx | Iny | Imm => 1,
            Abs | Abx | Aby | Ind => 2,
        }
    }

    pub fn get_operand(&self, cpu: &MOS6510) -> Operand {
        let (x, y) = (cpu.x, cpu.y);
        let addr_bytes = cpu.mem.read_slice(cpu.pc.wrapping_add(1), self.addr_size());

        use AddressingMode::*;
        match self {
            Imp | Akk => Operand::Implied,
            Abs => Operand::Address(concat(addr_bytes[0], addr_bytes[1])),
            Abx => Operand::Address(concat(addr_bytes[0], addr_bytes[1]).wrapping_add(x as u16)),
            Aby => Operand::Address(concat(addr_bytes[0], addr_bytes[1]).wrapping_add(y as u16)),
            Zpg => Operand::Address(addr_bytes[0] as u16),
            Zpx => Operand::Address(addr_bytes[0].wrapping_add(x) as u16),
            Zpy => Operand::Address(addr_bytes[0].wrapping_add(y) as u16),
            Rel => {
                let offset = addr_bytes[0];
                let sign_extend = if sign_bit(offset) { 0xff } else { 0x00 };
                let rel = u16::from_le_bytes([offset, sign_extend]);
                Operand::AddressOffset(rel)
            }
            Ind => {
                let indirect_addr = concat(addr_bytes[0], addr_bytes[1]);
                let indirect = cpu.mem.read_slice(indirect_addr, 2);
                Operand::Address(concat(indirect[0], indirect[1]))
            }
            Inx => {
                let start = addr_bytes[0].wrapping_add(x) as u16;
                let indirect = cpu.mem.read_slice(start, 2);
                Operand::Address(concat(indirect[0], indirect[1]))
            }
            Iny => {
                let start = addr_bytes[0] as u16;
                let indirect = cpu.mem.read_slice(start, 2);
                Operand::Address(concat(indirect[0], indirect[1]).wrapping_add(y as u16))
            }
            Imm => Operand::Byte(addr_bytes[0]),
        }
    }
}
