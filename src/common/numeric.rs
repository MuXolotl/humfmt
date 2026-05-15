/// Internal numeric input representation used by the number formatter.
///
/// Created via the `NumberLike` trait implementations. Users never construct
/// this type directly.
#[derive(Copy, Clone, Debug)]
pub enum NumericValue {
    /// Signed integer input (`i8`..`i128`, `isize`).
    Int(i128),
    /// Unsigned integer input (`u8`..`u128`, `usize`).
    UInt(u128),
    /// Floating-point input (`f32`, `f64`).
    Float(f64),
}
