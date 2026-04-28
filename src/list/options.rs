use crate::locale::{English, Locale};

/// Builder-style configuration for list formatting.
///
/// `humfmt` list formatting is intentionally minimal and predictable:
/// it joins slices into natural-language lists while respecting locale defaults,
/// with optional overrides.
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
pub struct ListOptions<L: Locale = English> {
    serial_comma: Option<bool>,
    conjunction: Option<&'static str>,
    locale: L,
}

impl ListOptions<English> {
    /// Creates default list formatting options (English).
    ///
    /// Defaults:
    /// - serial comma: locale default (English: enabled)
    /// - conjunction: locale default (English: `"and"`)
    /// - locale: `English`
    #[inline]
    pub fn new() -> Self {
        Self {
            serial_comma: None,
            conjunction: None,
            locale: English,
        }
    }
}

impl<L: Locale> Default for ListOptions<L> {
    #[inline]
    fn default() -> Self {
        Self {
            serial_comma: None,
            conjunction: None,
            locale: L::default(),
        }
    }
}

impl<L: Locale> ListOptions<L> {
    /// Enables the serial comma before the final list item.
    ///
    /// This is equivalent to `.serial_comma_enabled(true)`.
    #[inline]
    pub fn serial_comma(mut self) -> Self {
        self.serial_comma = Some(true);
        self
    }

    /// Configures serial-comma behavior with an explicit boolean.
    ///
    /// - `true`: `"a, b, and c"`
    /// - `false`: `"a, b and c"`
    #[inline]
    pub fn serial_comma_enabled(mut self, enabled: bool) -> Self {
        self.serial_comma = Some(enabled);
        self
    }

    /// Disables the serial comma before the final list item.
    ///
    /// This is equivalent to `.serial_comma_enabled(false)`.
    #[inline]
    pub fn no_serial_comma(mut self) -> Self {
        self.serial_comma = Some(false);
        self
    }

    /// Overrides the conjunction used to join the final list item.
    ///
    /// Example: `"plus"` produces `"a, b plus c"`.
    #[inline]
    pub fn conjunction(mut self, word: &'static str) -> Self {
        self.conjunction = Some(word);
        self
    }

    /// Switches the active locale for list formatting.
    ///
    /// Locale influences:
    /// - default conjunction (`and_word`)
    /// - default serial comma preference
    /// - list separator between items
    #[inline]
    pub fn locale<N: Locale>(self, locale: N) -> ListOptions<N> {
        ListOptions {
            serial_comma: self.serial_comma,
            conjunction: self.conjunction,
            locale,
        }
    }

    pub(crate) fn serial_comma_value(&self) -> bool {
        self.serial_comma
            .unwrap_or_else(|| self.locale.serial_comma())
    }

    pub(crate) fn locale_ref(&self) -> &L {
        &self.locale
    }

    pub(crate) fn conjunction_or<'a>(&'a self, fallback: &'a str) -> &'a str {
        self.conjunction.unwrap_or(fallback)
    }
}
