#[allow(clippy::upper_case_acronyms)]
pub enum Opcode {
    /* arithmetic */
    /// add with carry
    ADC,
    /// subtract with carry
    SBC,

    /* compare */
    /// compare with accumulator
    CMP,
    /// compare with X
    CPX,
    /// compare with Y
    CPY,

    /* logical */
    /// AND with accumulator
    AND,
    /// XOR with accumulator
    EOR,
    /// OR with accumulator
    ORA,
    /// bit test
    BIT,

    /* increment/decrement */
    /// decrement
    DEC,
    /// decrement X
    DEX,
    /// decrement Y
    DEY,
    /// increment
    INC,
    /// increment X
    INX,
    /// increment Y
    INY,

    /* shifts */
    /// arithmetic shift left
    ASL,
    /// logical shift right
    LSR,
    /// rotate left
    ROL,
    /// rotate right
    ROR,

    /* branching */
    /// branch on carry clear
    BCC,
    /// branch on carry set
    BCS,
    /// branch on equal (zero set)
    BEO,
    /// branch on minus (negative set)
    BMI,
    /// branch on not equal (zero clear)
    BNE,
    /// branch on plus (negative clear)
    BPL,
    /// branch on overflow clear
    BVC,
    /// branch on overflow set
    BVS,

    /* system functions */
    /// break / interrupt
    BRK,
    /// no operation
    NOP,

    /* jumps */
    /// jump
    JMP,
    /// jump subroutine
    JSR,

    /* returns */
    /// return from interrupt
    RTI,
    /// return from subroutine
    RTS,

    /* status flags */
    /// set carry
    SEC,
    /// set decimal
    SED,
    /// set interrupt disable
    SEI,
    /// clear carry
    CLC,
    /// clear decimal
    CLD,
    /// clear interrupt disable
    CLI,

    /* load/store */
    /// load accumulator
    LDA,
    /// load X
    LDX,
    /// load Y
    LDY,
    /// store accumulator
    STA,
    /// store X
    STX,
    /// store Y
    STY,

    /* push/pull */
    /// push accumulator
    PHA,
    /// push processor status (SR)
    PHP,
    /// pull accumulator
    PLA,
    /// pull processor status (SR)
    PLP,

    /* transfers */
    /// transfer accumulator to X
    TAX,
    /// transfer accumulator to Y
    TAY,
    /// transfer stack pointer to X
    TSX,
    /// transfer X to accumulator
    TXA,
    /// transfer X to stack pointer
    TXS,
    /// transfer Y to accumulator
    TYA,
}
