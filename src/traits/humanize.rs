/// Extension trait providing ergonomic `.human_*()` methods.
///
/// This trait is implemented for all types (`impl<T> Humanize for T {}`),
/// but each method is only available when the receiver implements the
/// corresponding `*Like` trait (`BytesLike`, `NumberLike`, etc.).
///
/// This keeps the API baby-simple:
///
/// ```rust
/// use humfmt::Humanize;
///
/// assert_eq!(1_500_000.human_number().to_string(), "1.5M");
/// assert_eq!(1536_u64.human_bytes().to_string(), "1.5KB");
/// assert_eq!(0.423_f64.human_percent().to_string(), "42.3%");
/// ```
pub trait Humanize: Sized {
    /// Formats this value as a human-readable byte size using default options.
    fn human_bytes(self) -> crate::bytes::BytesDisplay
    where
        Self: crate::bytes::BytesLike,
    {
        crate::bytes::bytes(self)
    }

    /// Formats this value as a human-readable byte size using custom options.
    fn human_bytes_with(self, options: crate::bytes::BytesOptions) -> crate::bytes::BytesDisplay
    where
        Self: crate::bytes::BytesLike,
    {
        crate::bytes::bytes_with(self, options)
    }

    /// Formats this duration as relative time using default options.
    ///
    /// Output is localized via the active locale in [`crate::DurationOptions`].
    fn human_ago(self) -> crate::ago::AgoDisplay
    where
        Self: crate::duration::DurationLike,
    {
        crate::ago::ago(self)
    }

    /// Formats this duration as relative time using custom duration options.
    fn human_ago_with<L: crate::locale::Locale>(
        self,
        options: crate::duration::DurationOptions<L>,
    ) -> crate::ago::AgoDisplay<L>
    where
        Self: crate::duration::DurationLike,
    {
        crate::ago::ago_with(self, options)
    }

    /// Formats this duration using default options.
    fn human_duration(self) -> crate::duration::DurationDisplay
    where
        Self: crate::duration::DurationLike,
    {
        crate::duration::duration(self)
    }

    /// Formats this duration using custom duration options.
    fn human_duration_with<L: crate::locale::Locale>(
        self,
        options: crate::duration::DurationOptions<L>,
    ) -> crate::duration::DurationDisplay<L>
    where
        Self: crate::duration::DurationLike,
    {
        crate::duration::duration_with(self, options)
    }

    /// Formats this numeric value as a compact human-readable number using default options.
    fn human_number(self) -> crate::number::NumberDisplay<crate::locale::English>
    where
        Self: crate::number::NumberLike,
    {
        crate::number::number(self)
    }

    /// Formats this numeric value as a compact human-readable number using custom options.
    fn human_number_with<L: crate::locale::Locale>(
        self,
        options: crate::number::NumberOptions<L>,
    ) -> crate::number::NumberDisplay<L>
    where
        Self: crate::number::NumberLike,
    {
        crate::number::number_with(self, options)
    }

    /// Formats this value as an ordinal number using the default locale (English).
    fn human_ordinal(self) -> crate::ordinal::OrdinalDisplay<crate::locale::English>
    where
        Self: crate::ordinal::OrdinalLike,
    {
        crate::ordinal::ordinal(self)
    }

    /// Formats this value as an ordinal number using a custom locale.
    fn human_ordinal_with<L: crate::locale::Locale>(
        self,
        locale: L,
    ) -> crate::ordinal::OrdinalDisplay<L>
    where
        Self: crate::ordinal::OrdinalLike,
    {
        crate::ordinal::ordinal_with(self, locale)
    }

    /// Formats this value as a human-readable percentage using default options.
    ///
    /// The input is a ratio: `1.0` means `100%`, `0.5` means `50%`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use humfmt::Humanize;
    ///
    /// assert_eq!(0.423_f64.human_percent().to_string(), "42.3%");
    /// assert_eq!(1.0_f64.human_percent().to_string(), "100%");
    /// ```
    fn human_percent(self) -> crate::percent::PercentDisplay<crate::locale::English>
    where
        Self: crate::percent::PercentLike,
    {
        crate::percent::percent(self)
    }

    /// Formats this value as a human-readable percentage using custom options.
    fn human_percent_with<L: crate::locale::Locale>(
        self,
        options: crate::percent::PercentOptions<L>,
    ) -> crate::percent::PercentDisplay<L>
    where
        Self: crate::percent::PercentLike,
    {
        crate::percent::percent_with(self, options)
    }
}

impl<T> Humanize for T {}
