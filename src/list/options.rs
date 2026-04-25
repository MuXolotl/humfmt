use crate::locale::{English, Locale};

/// Builder-style configuration for list formatting.
///
/// # Examples
///
/// ```rust
/// use humfmt::ListOptions;
///
/// let opts = ListOptions::new()
///     .serial_comma_enabled(false)
///     .conjunction("plus");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct ListOptions<L: Locale = English> {
    serial_comma: Option<bool>,
    conjunction: Option<&'static str>,
    locale: L,
}

impl ListOptions<English> {
    /// Creates default list formatting options.
    pub fn new() -> Self {
        Self {
            serial_comma: None,
            conjunction: None,
            locale: English,
        }
    }
}

impl<L: Locale> Default for ListOptions<L> {
    fn default() -> Self {
        Self {
            serial_comma: None,
            conjunction: None,
            locale: L::default(),
        }
    }
}

impl<L: Locale> ListOptions<L> {
    /// Enables a serial comma before the final list item.
    pub fn serial_comma(mut self) -> Self {
        self.serial_comma = Some(true);
        self
    }

    /// Configures serial-comma behavior with an explicit boolean.
    pub fn serial_comma_enabled(mut self, enabled: bool) -> Self {
        self.serial_comma = Some(enabled);
        self
    }

    /// Disables the serial comma before the final list item.
    pub fn no_serial_comma(mut self) -> Self {
        self.serial_comma = Some(false);
        self
    }

    /// Overrides the conjunction used to join the final list item.
    pub fn conjunction(mut self, word: &'static str) -> Self {
        self.conjunction = Some(word);
        self
    }

    /// Switches the active locale.
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
