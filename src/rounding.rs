/// Specifies how numerical values should be rounded.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum RoundingMode {
    /// Round to the nearest value, with ties rounding away from zero (default).
    #[default]
    HalfUp,
    /// Round towards negative infinity (down).
    Floor,
    /// Round towards positive infinity (up).
    Ceil,
}
