use addressing::AddrMode::*;
use opcode::Opcode::*;

mod addressing;
mod opcode;

pub static INSTRUCTIONS: [Option<(opcode::Opcode, addressing::AddrMode)>; 256] = [
    Some((BRK, Imp)), // 0x00
    Some((ORA, Inx)), // 0x01
    None,             // 0x02
    None,             // 0x03
    None,             // 0x04
    Some((ORA, Zpg)), // 0x05
    Some((ASL, Zpg)), // 0x06
    None,             // 0x07
    Some((PHP, Imp)), // 0x08
    Some((ORA, Imm)), // 0x09
    Some((ASL, Akk)), // 0x0a
    None,             // 0x0b
    None,             // 0x0c
    Some((ORA, Abs)), // 0x0d
    Some((ASL, Abs)), // 0x0e
    None,             // 0x0f
    Some((BPL, Rel)), // 0x10
    Some((ORA, Iny)), // 0x11
    None,             // 0x12
    None,             // 0x13
    None,             // 0x14
    Some((ORA, Zpx)), // 0x15
    Some((ASL, Zpx)), // 0x16
    None,             // 0x17
    Some((CLC, Imp)), // 0x18
    Some((ORA, Aby)), // 0x19
    None,             // 0x1a
    None,             // 0x1b
    None,             // 0x1c
    Some((ORA, Abx)), // 0x1d
    Some((ASL, Abx)), // 0x1e
    None,             // 0x1f
    Some((JSR, Abs)), // 0x20
    Some((AND, Inx)), // 0x21
    None,             // 0x22
    None,             // 0x23
    Some((BIT, Zpg)), // 0x24
    Some((AND, Zpg)), // 0x25
    Some((ROL, Zpg)), // 0x26
    None,             // 0x27
    Some((PLP, Imp)), // 0x28
    Some((AND, Imm)), // 0x29
    Some((ROL, Akk)), // 0x2a
    None,             // 0x2b
    Some((BIT, Abs)), // 0x2c
    Some((AND, Abs)), // 0x2d
    Some((ROL, Abs)), // 0x2e
    None,             // 0x2f
    Some((BMI, Rel)), // 0x30
    Some((AND, Iny)), // 0x31
    None,             // 0x32
    None,             // 0x33
    None,             // 0x34
    Some((AND, Zpx)), // 0x35
    Some((ROL, Zpx)), // 0x36
    None,             // 0x37
    Some((SEC, Imp)), // 0x38
    Some((AND, Aby)), // 0x39
    None,             // 0x3a
    None,             // 0x3b
    None,             // 0x3c
    Some((AND, Abx)), // 0x3d
    Some((ROL, Abx)), // 0x3e
    None,             // 0x3f
    Some((RTI, Imp)), // 0x40
    Some((EOR, Inx)), // 0x41
    None,             // 0x42
    None,             // 0x43
    None,             // 0x44
    Some((EOR, Zpg)), // 0x45
    Some((LSR, Zpg)), // 0x46
    None,             // 0x47
    Some((PHA, Imp)), // 0x48
    Some((EOR, Imm)), // 0x49
    Some((LSR, Akk)), // 0x4a
    None,             // 0x4b
    Some((JMP, Abs)), // 0x4c
    Some((EOR, Abs)), // 0x4d
    Some((LSR, Abs)), // 0x4e
    None,             // 0x4f
    Some((BVC, Rel)), // 0x50
    Some((EOR, Iny)), // 0x51
    None,             // 0x52
    None,             // 0x53
    None,             // 0x54
    Some((EOR, Zpx)), // 0x55
    Some((LSR, Zpx)), // 0x56
    None,             // 0x57
    Some((CLI, Imp)), // 0x58
    Some((EOR, Aby)), // 0x59
    None,             // 0x5a
    None,             // 0x5b
    None,             // 0x5c
    Some((EOR, Abx)), // 0x5d
    Some((LSR, Abx)), // 0x5e
    None,             // 0x5f
    Some((RTS, Imp)), // 0x60
    Some((ADC, Inx)), // 0x61
    None,             // 0x62
    None,             // 0x63
    None,             // 0x64
    Some((ADC, Zpg)), // 0x65
    Some((ROR, Zpg)), // 0x66
    None,             // 0x67
    Some((PLA, Imp)), // 0x68
    Some((ADC, Imm)), // 0x69
    Some((ROR, Akk)), // 0x6a
    None,             // 0x6b
    Some((JMP, Ind)), // 0x6c
    Some((ADC, Abs)), // 0x6d
    Some((ROR, Abs)), // 0x6e
    None,             // 0x6f
    Some((BVS, Rel)), // 0x70
    Some((ADC, Iny)), // 0x71
    None,             // 0x72
    None,             // 0x73
    None,             // 0x74
    Some((ADC, Zpx)), // 0x75
    Some((ROR, Zpx)), // 0x76
    None,             // 0x77
    Some((SEI, Imp)), // 0x78
    Some((ADC, Aby)), // 0x79
    None,             // 0x7a
    None,             // 0x7b
    None,             // 0x7c
    Some((ADC, Abx)), // 0x7d
    Some((ROR, Abx)), // 0x7e
    None,             // 0x7f
    None,             // 0x80
    Some((STA, Inx)), // 0x81
    None,             // 0x82
    None,             // 0x83
    Some((STY, Zpg)), // 0x84
    Some((STA, Zpg)), // 0x85
    Some((STX, Zpg)), // 0x86
    None,             // 0x87
    Some((DEY, Imp)), // 0x88
    None,             // 0x89
    Some((TXA, Imp)), // 0x8a
    None,             // 0x8b
    Some((STY, Abs)), // 0x8c
    Some((STA, Abs)), // 0x8d
    Some((STX, Abs)), // 0x8e
    None,             // 0x8f
    Some((BCC, Rel)), // 0x90
    Some((STA, Iny)), // 0x91
    None,             // 0x92
    None,             // 0x93
    Some((STY, Zpx)), // 0x94
    Some((STA, Zpx)), // 0x95
    Some((STX, Zpy)), // 0x96
    None,             // 0x97
    Some((TYA, Imp)), // 0x98
    Some((STA, Aby)), // 0x99
    Some((TXS, Imp)), // 0x9a
    None,             // 0x9b
    None,             // 0x9c
    Some((STA, Abx)), // 0x9d
    None,             // 0x9e
    None,             // 0x9f
    Some((LDY, Imm)), // 0xa0
    Some((LDA, Inx)), // 0xa1
    Some((LDX, Imm)), // 0xa2
    None,             // 0xa3
    Some((LDY, Zpg)), // 0xa4
    Some((LDA, Zpg)), // 0xa5
    Some((LDX, Zpg)), // 0xa6
    None,             // 0xa7
    Some((TAY, Imp)), // 0xa8
    Some((LDA, Imm)), // 0xa9
    Some((TAX, Imp)), // 0xaa
    None,             // 0xab
    Some((LDY, Abs)), // 0xac
    Some((LDA, Abs)), // 0xad
    Some((LDX, Abs)), // 0xae
    None,             // 0xaf
    Some((BCS, Rel)), // 0xb0
    Some((LDA, Iny)), // 0xb1
    None,             // 0xb2
    None,             // 0xb3
    Some((LDY, Zpx)), // 0xb4
    Some((LDA, Zpx)), // 0xb5
    Some((LDX, Zpy)), // 0xb6
    None,             // 0xb7
    Some((CLV, Imp)), // 0xb8
    Some((LDA, Aby)), // 0xb9
    Some((TSX, Imp)), // 0xba
    None,             // 0xbb
    Some((LDY, Abx)), // 0xbc
    Some((LDA, Abx)), // 0xbd
    Some((LDX, Aby)), // 0xbe
    None,             // 0xbf
    Some((CPY, Imm)), // 0xc0
    Some((CMP, Inx)), // 0xc1
    None,             // 0xc2
    None,             // 0xc3
    Some((CPY, Zpg)), // 0xc4
    Some((CMP, Zpg)), // 0xc5
    Some((DEC, Zpg)), // 0xc6
    None,             // 0xc7
    Some((INY, Imp)), // 0xc8
    Some((CMP, Imm)), // 0xc9
    Some((DEX, Imp)), // 0xca
    None,             // 0xcb
    Some((CPY, Abs)), // 0xcc
    Some((CMP, Abs)), // 0xcd
    Some((DEC, Abs)), // 0xce
    None,             // 0xcf
    Some((BNE, Rel)), // 0xd0
    Some((CMP, Iny)), // 0xd1
    None,             // 0xd2
    None,             // 0xd3
    None,             // 0xd4
    Some((CMP, Zpx)), // 0xd5
    Some((DEC, Zpx)), // 0xd6
    None,             // 0xd7
    Some((CLD, Imp)), // 0xd8
    Some((CMP, Aby)), // 0xd9
    None,             // 0xda
    None,             // 0xdb
    None,             // 0xdc
    Some((CMP, Abx)), // 0xdd
    Some((DEC, Abx)), // 0xde
    None,             // 0xdf
    Some((CPX, Imm)), // 0xe0
    Some((SBC, Inx)), // 0xe1
    None,             // 0xe2
    None,             // 0xe3
    Some((CPX, Zpg)), // 0xe4
    Some((SBC, Zpg)), // 0xe5
    Some((INC, Zpg)), // 0xe6
    None,             // 0xe7
    Some((INX, Imp)), // 0xe8
    Some((SBC, Imm)), // 0xe9
    Some((NOP, Imp)), // 0xea
    None,             // 0xeb
    Some((CPX, Abs)), // 0xec
    Some((SBC, Abs)), // 0xed
    Some((INC, Abs)), // 0xee
    None,             // 0xef
    Some((BEQ, Rel)), // 0xf0
    Some((SBC, Iny)), // 0xf1
    None,             // 0xf2
    None,             // 0xf3
    None,             // 0xf4
    Some((SBC, Zpx)), // 0xf5
    Some((INC, Zpx)), // 0xf6
    None,             // 0xf7
    Some((SED, Imp)), // 0xf8
    Some((SBC, Aby)), // 0xf9
    None,             // 0xfa
    None,             // 0xfb
    None,             // 0xfc
    Some((SBC, Abx)), // 0xfd
    Some((INC, Abx)), // 0xfe
    None,             // 0xff
];
