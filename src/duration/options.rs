/// Builder-style configuration for duration formatting.
///
/// # Examples
///
/// ```rust
/// use humfmt::DurationOptions;
///
/// let opts = DurationOptions::new().max_units(3).long_units();
/// ```
#[derive(Copy, Clone, Debug)]
pub struct DurationOptions {
    max_units: u8,
    long_units: bool,
}

impl DurationOptions {
    /// Creates default duration formatting options.
    pub fn new() -> Self {
        Self {
            max_units: 2,
            long_units: false,
        }
    }

    /// Limits how many non-zero units are rendered.
    pub fn max_units(mut self, n: u8) -> Self {
        self.max_units = n.clamp(1, 4);
        self
    }

    /// Uses long English unit labels like `hour` instead of `h`.
    pub fn long_units(mut self) -> Self {
        self.long_units = true;
        self
    }

    pub(crate) fn max_units_value(&self) -> u8 {
        self.max_units
    }

    pub(crate) fn long_units_value(&self) -> bool {
        self.long_units
    }
}

impl Default for DurationOptions {
    fn default() -> Self {
        Self::new()
    }
}
