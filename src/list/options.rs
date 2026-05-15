/// Builder-style configuration for list formatting.
///
/// `humfmt` list formatting is intentionally minimal and predictable:
/// it joins slices into natural-language lists with optional overrides.
///
/// # Examples
///
/// ```rust
/// use humfmt::ListOptions;
///
/// let opts = ListOptions::new()
///     .serial_comma_enabled(false)
///     .conjunction("plus");
///
/// assert_eq!(
///     humfmt::list_with(&["red", "green", "blue"], opts).to_string(),
///     "red, green plus blue"
/// );
/// ```
#[derive(Copy, Clone, Debug)]
pub struct ListOptions {
    pub(crate) serial_comma: bool,
    pub(crate) conjunction: &'static str,
    pub(crate) separator: &'static str,
}

impl ListOptions {
    /// Creates default list formatting options.
    ///
    /// Defaults:
    /// - serial comma: enabled (Oxford comma)
    /// - conjunction: `"and"`
    /// - separator: `", "`
    #[inline]
    pub const fn new() -> Self {
        Self {
            serial_comma: true,
            conjunction: "and",
            separator: ", ",
        }
    }

    /// Enables the serial comma before the final list item.
    ///
    /// Equivalent to `.serial_comma_enabled(true)`.
    #[inline]
    pub const fn serial_comma(mut self) -> Self {
        self.serial_comma = true;
        self
    }

    /// Configures serial-comma behavior with an explicit boolean.
    ///
    /// - `true`: `"a, b, and c"`
    /// - `false`: `"a, b and c"`
    #[inline]
    pub const fn serial_comma_enabled(mut self, enabled: bool) -> Self {
        self.serial_comma = enabled;
        self
    }

    /// Disables the serial comma before the final list item.
    ///
    /// Equivalent to `.serial_comma_enabled(false)`.
    #[inline]
    pub const fn no_serial_comma(mut self) -> Self {
        self.serial_comma = false;
        self
    }

    /// Overrides the conjunction used to join the final list item.
    ///
    /// Default is `"and"`.
    ///
    /// Example: `"plus"` produces `"a, b plus c"`.
    #[inline]
    pub const fn conjunction(mut self, word: &'static str) -> Self {
        self.conjunction = word;
        self
    }

    /// Overrides the separator placed between list items.
    ///
    /// Default is `", "`.
    ///
    /// Note: serial comma is only injected when the separator is comma-style.
    /// Custom separators like `" | "` will not get a serial comma even if
    /// enabled.
    #[inline]
    pub const fn separator(mut self, sep: &'static str) -> Self {
        self.separator = sep;
        self
    }
}

impl Default for ListOptions {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
