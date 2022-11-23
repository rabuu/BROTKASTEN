/// Status flags for the `P` register
pub enum StatusFlag {
    /// negative
    N = 1 << 7,
    /// overflow
    V = 1 << 6,
    /// unused bit
    Reserved = 1 << 5,
    /// break
    B = 1 << 4,
    /// decimal
    D = 1 << 3,
    /// interrupt disable
    I = 1 << 2,
    /// zero
    Z = 1 << 1,
    /// carry
    C = 1 << 0,
}
