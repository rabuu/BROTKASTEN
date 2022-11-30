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
