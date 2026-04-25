/// Builder-style configuration for byte-size formatting.
///
/// # Examples
///
/// ```rust
/// use humfmt::BytesOptions;
///
/// let opts = BytesOptions::new().binary().precision(2);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct BytesOptions {
    precision: u8,
    binary: bool,
    long_units: bool,
}

impl BytesOptions {
    /// Creates default decimal byte formatting options.
    pub fn new() -> Self {
        Self {
            precision: 1,
            binary: false,
            long_units: false,
        }
    }

    /// Sets decimal precision for scaled values.
    pub fn precision(mut self, n: u8) -> Self {
        self.precision = n.min(6);
        self
    }

    /// Uses binary units like `KiB` instead of decimal `KB`.
    pub fn binary(mut self) -> Self {
        self.binary = true;
        self
    }

    /// Uses long unit labels like `kilobytes` instead of `KB`.
    pub fn long_units(mut self) -> Self {
        self.long_units = true;
        self
    }

    pub(crate) fn precision_value(&self) -> u8 {
        self.precision
    }

    pub(crate) fn binary_value(&self) -> bool {
        self.binary
    }

    pub(crate) fn long_units_value(&self) -> bool {
        self.long_units
    }
}

impl Default for BytesOptions {
    fn default() -> Self {
        Self::new()
    }
}
