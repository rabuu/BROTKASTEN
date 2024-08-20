bitflags::bitflags! {
    /// Status flags for the `P` register
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct Flags: u8 {
    /// carry flag
    const C = 0b00000001;

    /// zero flag
    const Z = 0b00000010;

    /// interrupt disable flag
    const I = 0b00000100;

    /// decimal flag
    const D = 0b00001000;

    /// break flag
    const B = 0b00010000;

    /// unused bit
    const UNUSED = 0b00100000;

    /// overflow flag
    const V = 0b01000000;

    /// negative flag
    const N = 0b10000000;
    }
}

impl Flags {
    /// Overwrites bits in `mask` with 1s if `value` is true, otherwise 0s
    pub fn overwrite(&mut self, mask: Self, value: bool) {
        if value {
            *self |= mask;
        } else {
            *self &= !mask;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overwrite() {
        let mut f = Flags::empty();
        f.overwrite(Flags::B | Flags::N, true);
        assert_eq!(f, Flags::N | Flags::B);

        f.overwrite(Flags::N, false);
        assert_eq!(f, Flags::B);
    }
}
