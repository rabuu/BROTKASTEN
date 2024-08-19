use tracing_subscriber::prelude::*;

fn main() {
    let fmt_layer = tracing_subscriber::fmt::layer().without_time().pretty();
    let filter_layer = tracing_subscriber::filter::LevelFilter::DEBUG;
    // let filter_layer = tracing_subscriber::filter::EnvFilter...;
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    // taken from https://github.com/mre/mos6502 README

    // calculate gcd of 56 and 49 using Euclid's algorithm

    let zero_page_data = [56, 49];

    let program = [
        // .algo
        0xa5, 0x00, // Load from F to A
        // .algo_
        0x38, // Set carry flag
        0xe5, 0x01, // Substract S from number in A (from F)
        0xf0, 0x07, // Jump to .end if diff is zero
        0x30, 0x08, // Jump to .swap if diff is negative
        0x85, 0x00, // Load A to F
        0x4c, 0x12, 0x00, // Jump to .algo_
        // .end
        0xa5, 0x00, // Load from S to A
        0xff, // .swap
        0xa6, 0x00, // load F to X
        0xa4, 0x01, // load S to Y
        0x86, 0x01, // Store X to F
        0x84, 0x00, // Store Y to S
        0x4c, 0x10, 0x00, // Jump to .algo
    ];

    let mut memory = brotkasten::memory::Memory::new();
    memory.write_slice(0x00, &zero_page_data).unwrap();
    memory.write_slice(0x10, &program).unwrap();

    let mut cpu = brotkasten::cpu::MOS6510::new(memory);
    cpu.pc = 0x10;

    cpu.run();

    assert_eq!(7, cpu.acc);
}
