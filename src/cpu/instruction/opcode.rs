#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    /* transfer */
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

    /* stack */
    /// push accumulator
    PHA,
    /// push processor status (SR)
    PHP,
    /// pull accumulator
    PLA,
    /// pull processor status (SR)
    PLP,

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

    /* arithmetic */
    /// add with carry
    ADC,
    /// subtract with carry
    SBC,

    /* logical */
    /// AND with accumulator
    AND,
    /// XOR with accumulator
    EOR,
    /// OR with accumulator
    ORA,

    /* shift/rotate */
    /// arithmetic shift left
    ASL,
    /// logical shift right
    LSR,
    /// rotate left
    ROL,
    /// rotate right
    ROR,

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
    /// clear overflow
    CLV,

    /* comparison */
    /// compare with accumulator
    CMP,
    /// compare with X
    CPX,
    /// compare with Y
    CPY,

    /* branching */
    /// branch on carry clear
    BCC,
    /// branch on carry set
    BCS,
    /// branch on equal (zero set)
    BEQ,
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

    /* jump */
    /// jump
    JMP,
    /// jump subroutine
    JSR,
    /// return from subroutine
    RTS,

    /* interrupt */
    /// break / interrupt
    BRK,
    /// return from interrupt
    RTI,

    /* other */
    /// bit test
    BIT,
    /// no operation
    NOP,
}
