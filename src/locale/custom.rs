use super::{english, DurationUnit, Locale};

#[cfg(feature = "polish")]
use super::polish;

#[cfg(feature = "russian")]
use super::russian;

/// Function pointer type used to resolve compact suffixes.
///
/// This hook is value-aware and can be used for locale-specific inflection rules.
///
/// Arguments:
/// - `idx`: suffix index (0 = none, 1 = thousand, 2 = million, ...).
/// - `scaled`: the rendered scaled value (e.g. `1.0`, `2.5`).
/// - `long`: whether long-form labels are requested.
///
/// Returned strings must be `'static` and are written directly into the output.
pub type CompactSuffixFn = fn(idx: usize, scaled: f64, long: bool) -> &'static str;

/// Function pointer type used to resolve duration unit labels.
///
/// Arguments:
/// - `unit`: unit kind.
/// - `count`: quantity of units.
/// - `long`: long-form vs short-form labels.
///
/// Returned strings must be `'static` and are written directly into the output.
pub type DurationUnitFn = fn(unit: DurationUnit, count: u128, long: bool) -> &'static str;

/// Function pointer type used to resolve ordinal suffixes.
///
/// Returned strings must be `'static` and are written directly into the output.
pub type OrdinalSuffixFn = fn(n: u128) -> &'static str;

const COMPACT_SUFFIX_CAPACITY: usize = 12;

/// Builder-style locale for ad hoc compact-number and ordinal customization.
///
/// `CustomLocale` is designed for:
/// - minimal allocation (returns `'static` labels),
/// - cheap copying (the type is `Copy`),
/// - easy customization without implementing [`Locale`] manually.
///
/// Suffix strings are rendered as-is. Long-form suffixes should include any
/// leading whitespace you want in the final output.
///
/// # Examples
///
/// ```rust
/// use humfmt::{
///     locale::CustomLocale,
///     number_with,
///     ordinal_with,
///     NumberOptions,
/// };
///
/// fn ordinal_marker(_: u128) -> &'static str {
///     "o"
/// }
///
/// let locale = CustomLocale::english()
///     .short_suffix(1, "k")
///     .separators(',', '.')
///     .ordinal_suffix_fn(ordinal_marker);
///
/// assert_eq!(
///     number_with(15_320, NumberOptions::new().locale(locale)).to_string(),
///     "15,3k"
/// );
/// assert_eq!(ordinal_with(7, locale).to_string(), "7o");
/// ```
#[derive(Copy, Clone, Debug)]
pub struct CustomLocale {
    short_suffixes: [&'static str; COMPACT_SUFFIX_CAPACITY],
    long_suffixes: [&'static str; COMPACT_SUFFIX_CAPACITY],
    short_overrides: [bool; COMPACT_SUFFIX_CAPACITY],
    long_overrides: [bool; COMPACT_SUFFIX_CAPACITY],
    compact_suffix_fn: Option<CompactSuffixFn>,
    duration_unit_fn: DurationUnitFn,
    ordinal_suffix_fn: OrdinalSuffixFn,
    max_compact_suffix_index: usize,
    decimal_separator: char,
    group_separator: char,
    list_separator: &'static str,
    and_word: &'static str,
    serial_comma: bool,
    ago_word: &'static str,
}

impl CustomLocale {
    /// Creates a customization-friendly locale initialized from English.
    pub fn english() -> Self {
        Self {
            short_suffixes: english::SHORT_SUFFIXES,
            long_suffixes: english::LONG_SUFFIXES,
            short_overrides: [false; COMPACT_SUFFIX_CAPACITY],
            long_overrides: [false; COMPACT_SUFFIX_CAPACITY],
            compact_suffix_fn: None,
            duration_unit_fn: english::duration_unit,
            ordinal_suffix_fn: english::ordinal_suffix,
            max_compact_suffix_index: english::MAX_COMPACT_SUFFIX_INDEX,
            decimal_separator: '.',
            group_separator: ',',
            list_separator: ", ",
            and_word: "and",
            serial_comma: true,
            ago_word: "ago",
        }
    }

    /// Creates a customization-friendly locale initialized from Russian.
    #[cfg(feature = "russian")]
    pub fn russian() -> Self {
        Self {
            short_suffixes: russian::SHORT_SUFFIXES,
            long_suffixes: russian::LONG_SUFFIXES,
            short_overrides: [false; COMPACT_SUFFIX_CAPACITY],
            long_overrides: [false; COMPACT_SUFFIX_CAPACITY],
            compact_suffix_fn: Some(russian::compact_suffix_for),
            duration_unit_fn: russian::duration_unit,
            ordinal_suffix_fn: russian::ordinal_suffix,
            max_compact_suffix_index: russian::MAX_COMPACT_SUFFIX_INDEX,
            decimal_separator: ',',
            group_separator: ' ',
            list_separator: ", ",
            and_word: "и",
            serial_comma: false,
            ago_word: "назад",
        }
    }

    /// Creates a customization-friendly locale initialized from Polish.
    #[cfg(feature = "polish")]
    pub fn polish() -> Self {
        Self {
            short_suffixes: polish::SHORT_SUFFIXES,
            long_suffixes: polish::LONG_SUFFIXES,
            short_overrides: [false; COMPACT_SUFFIX_CAPACITY],
            long_overrides: [false; COMPACT_SUFFIX_CAPACITY],
            compact_suffix_fn: Some(polish::compact_suffix_for),
            duration_unit_fn: polish::duration_unit,
            ordinal_suffix_fn: polish::ordinal_suffix,
            max_compact_suffix_index: polish::MAX_COMPACT_SUFFIX_INDEX,
            decimal_separator: ',',
            group_separator: ' ',
            list_separator: ", ",
            and_word: "i",
            serial_comma: false,
            ago_word: "temu",
        }
    }

    /// Overrides the maximum compact unit index used for scaling.
    ///
    /// Useful for capping the output to a specific maximum unit (e.g. stopping at
    /// millions so 1 billion renders as `1000M`).
    ///
    /// To completely disable compact scaling, use [`crate::NumberOptions::compact`]
    /// instead.
    pub fn max_compact_suffix_index(mut self, idx: usize) -> Self {
        self.max_compact_suffix_index = idx.min(COMPACT_SUFFIX_CAPACITY - 1);
        self
    }

    /// Overrides the decimal separator used when rendering numbers.
    pub fn decimal_separator(mut self, separator: char) -> Self {
        self.decimal_separator = separator;
        self
    }

    /// Overrides the thousands-group separator used when rendering numbers.
    pub fn group_separator(mut self, separator: char) -> Self {
        self.group_separator = separator;
        self
    }

    /// Overrides both decimal and group separators at once.
    pub fn separators(mut self, decimal: char, group: char) -> Self {
        self.decimal_separator = decimal;
        self.group_separator = group;
        self
    }

    /// Overrides the locale word used for conjunction-style output.
    pub fn and_word(mut self, word: &'static str) -> Self {
        self.and_word = word;
        self
    }

    /// Overrides the separator placed between list items.
    pub fn list_separator(mut self, separator: &'static str) -> Self {
        self.list_separator = separator;
        self
    }

    /// Overrides whether list formatting should use a serial comma by default.
    pub fn serial_comma(mut self, enabled: bool) -> Self {
        self.serial_comma = enabled;
        self
    }

    /// Overrides the locale word used for relative-time output.
    pub fn ago_word(mut self, word: &'static str) -> Self {
        self.ago_word = word;
        self
    }

    /// Overrides one short compact suffix slot.
    ///
    /// Index `1` corresponds to thousand, `2` to million, and so on.
    pub fn short_suffix(mut self, idx: usize, suffix: &'static str) -> Self {
        if idx < COMPACT_SUFFIX_CAPACITY {
            self.short_suffixes[idx] = suffix;
            self.short_overrides[idx] = true;
        }
        self
    }

    /// Overrides one long compact suffix slot.
    ///
    /// Index `1` corresponds to thousand, `2` to million, and so on.
    /// Include any leading whitespace you want preserved in the rendered output.
    pub fn long_suffix(mut self, idx: usize, suffix: &'static str) -> Self {
        if idx < COMPACT_SUFFIX_CAPACITY {
            self.long_suffixes[idx] = suffix;
            self.long_overrides[idx] = true;
        }
        self
    }

    /// Installs a value-aware compact suffix resolver.
    pub fn compact_suffix_fn(mut self, suffix_fn: CompactSuffixFn) -> Self {
        self.compact_suffix_fn = Some(suffix_fn);
        self
    }

    /// Installs a custom ordinal suffix resolver.
    pub fn ordinal_suffix_fn(mut self, suffix_fn: OrdinalSuffixFn) -> Self {
        self.ordinal_suffix_fn = suffix_fn;
        self
    }

    /// Installs a custom duration-unit resolver.
    pub fn duration_unit_fn(mut self, unit_fn: DurationUnitFn) -> Self {
        self.duration_unit_fn = unit_fn;
        self
    }
}

impl Default for CustomLocale {
    fn default() -> Self {
        Self::english()
    }
}

impl Locale for CustomLocale {
    fn compact_suffix(&self, idx: usize, long: bool) -> &'static str {
        if idx >= COMPACT_SUFFIX_CAPACITY {
            return "";
        }

        if long {
            self.long_suffixes[idx]
        } else {
            self.short_suffixes[idx]
        }
    }

    fn compact_suffix_for(&self, idx: usize, scaled: f64, long: bool) -> &'static str {
        if idx < COMPACT_SUFFIX_CAPACITY {
            if long && self.long_overrides[idx] {
                return self.long_suffixes[idx];
            }

            if !long && self.short_overrides[idx] {
                return self.short_suffixes[idx];
            }
        }

        if let Some(suffix_fn) = self.compact_suffix_fn {
            return suffix_fn(idx, scaled, long);
        }

        self.compact_suffix(idx, long)
    }

    fn max_compact_suffix_index(&self) -> usize {
        self.max_compact_suffix_index
    }

    fn decimal_separator(&self) -> char {
        self.decimal_separator
    }

    fn group_separator(&self) -> char {
        self.group_separator
    }

    fn and_word(&self) -> &'static str {
        self.and_word
    }

    fn list_separator(&self) -> &'static str {
        self.list_separator
    }

    fn serial_comma(&self) -> bool {
        self.serial_comma
    }

    fn ago_word(&self) -> &'static str {
        self.ago_word
    }

    fn duration_unit(&self, unit: DurationUnit, count: u128, long: bool) -> &'static str {
        (self.duration_unit_fn)(unit, count, long)
    }

    fn ordinal_suffix(&self, n: u128) -> &'static str {
        (self.ordinal_suffix_fn)(n)
    }
}
