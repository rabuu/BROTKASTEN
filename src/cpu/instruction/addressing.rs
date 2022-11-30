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
}

#[derive(Debug)]
pub enum AddrOperand {
    Implied,
    Immediate(u8),
    Relative(u16),
    MemAddr(u16),
}
