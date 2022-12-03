use crate::cpu::MOS6510;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddrMode {
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

impl AddrMode {
    pub const fn addr_size(&self) -> u16 {
        match self {
            AddrMode::Imp => 0,
            AddrMode::Abs => 2,
            AddrMode::Abx => 2,
            AddrMode::Aby => 2,
            AddrMode::Zpg => 1,
            AddrMode::Zpx => 1,
            AddrMode::Zpy => 1,
            AddrMode::Rel => 1,
            AddrMode::Akk => 0,
            AddrMode::Ind => 2,
            AddrMode::Inx => 1,
            AddrMode::Iny => 1,
            AddrMode::Imm => 1,
        }
    }

    pub fn get_operand(&self, cpu: &MOS6510) -> AddrOperand {
        let (x, y) = (cpu.x, cpu.y);
        let addr_bytes = cpu.ram.read_slice(cpu.pc.wrapping_add(1), self.addr_size());

        use AddrMode::*;
        match self {
            Imp | Akk => AddrOperand::Implied,
            Abs => {
                let addr = construct_addr(addr_bytes[0], addr_bytes[1]);
                let val = cpu.ram.read(addr);
                AddrOperand::Value(val)
            }
            Abx => {
                let addr = construct_addr(addr_bytes[0], addr_bytes[1]).wrapping_add(x as u16);
                let val = cpu.ram.read(addr);
                AddrOperand::Value(val)
            }
            Aby => {
                let addr = construct_addr(addr_bytes[0], addr_bytes[1]).wrapping_add(y as u16);
                let val = cpu.ram.read(addr);
                AddrOperand::Value(val)
            }
            Zpg => {
                let addr = addr_bytes[0] as u16;
                let val = cpu.ram.read(addr);
                AddrOperand::Value(val)
            }
            Zpx => {
                let addr = addr_bytes[0].wrapping_add(x) as u16;
                let val = cpu.ram.read(addr);
                AddrOperand::Value(val)
            }
            Zpy => {
                let addr = addr_bytes[0].wrapping_add(y) as u16;
                let val = cpu.ram.read(addr);
                AddrOperand::Value(val)
            }
            Rel => {
                let offset = addr_bytes[0];
                // a little awkward number magic
                let sign_extend = if offset & 0x80 == 0x80 { 0xff } else { 0 };
                let rel = u16::from_le_bytes([offset, sign_extend]);
                AddrOperand::Relative(rel)
            }
            Ind => {
                let indirect_addr = construct_addr(addr_bytes[0], addr_bytes[1]);
                let indirect = cpu.ram.read_slice(indirect_addr, 2);
                let addr = construct_addr(indirect[0], indirect[1]);
                let val = cpu.ram.read(addr);
                AddrOperand::Value(val)
            }
            Inx => {
                let start = addr_bytes[0].wrapping_add(x) as u16;
                let indirect = cpu.ram.read_slice(start, 2);
                let addr = construct_addr(indirect[0], indirect[1]);
                let val = cpu.ram.read(addr);
                AddrOperand::Value(val)
            }
            Iny => {
                let start = addr_bytes[0] as u16;
                let indirect = cpu.ram.read_slice(start, 2);
                let addr = construct_addr(indirect[0], indirect[1]).wrapping_add(y as u16);
                let val = cpu.ram.read(addr);
                AddrOperand::Value(val)
            }
            Imm => AddrOperand::Value(addr_bytes[0]),
        }
    }
}

#[derive(Debug)]
pub enum AddrOperand {
    Implied,
    Value(u8),
    Relative(u16),
}

fn construct_addr(first: u8, second: u8) -> u16 {
    first as u16 + ((second as u16) << 8)
}
