/// Extension trait providing ergonomic `.human_*()` methods.
///
/// This trait is implemented for **all** types via a blanket impl
/// (`impl<T> Humanize for T {}`), but each method is only available when the
/// receiver implements the corresponding `*Like` trait (`BytesLike`,
/// `NumberLike`, etc.) — enforced via per-method `where` bounds.
///
/// This is a deliberate API design choice: the blanket impl keeps the import
/// surface tiny (`use humfmt::Humanize;` brings everything in), while the
/// `where` bounds keep the methods type-safe — calling
/// `"hello".human_number()` is a compile error, not a runtime surprise.
///
/// # Examples
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
    fn human_ago(self) -> crate::ago::AgoDisplay
    where
        Self: crate::duration::DurationLike,
    {
        crate::ago::ago(self)
    }

    /// Formats this duration as relative time using custom duration options.
    fn human_ago_with(self, options: crate::duration::DurationOptions) -> crate::ago::AgoDisplay
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
    fn human_duration_with(
        self,
        options: crate::duration::DurationOptions,
    ) -> crate::duration::DurationDisplay
    where
        Self: crate::duration::DurationLike,
    {
        crate::duration::duration_with(self, options)
    }

    /// Formats this numeric value as a compact human-readable number using default options.
    fn human_number(self) -> crate::number::NumberDisplay
    where
        Self: crate::number::NumberLike,
    {
        crate::number::number(self)
    }

    /// Formats this numeric value as a compact human-readable number using custom options.
    fn human_number_with(
        self,
        options: crate::number::NumberOptions,
    ) -> crate::number::NumberDisplay
    where
        Self: crate::number::NumberLike,
    {
        crate::number::number_with(self, options)
    }

    /// Formats this value as an ordinal number.
    fn human_ordinal(self) -> crate::ordinal::OrdinalDisplay
    where
        Self: crate::ordinal::OrdinalLike,
    {
        crate::ordinal::ordinal(self)
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
    fn human_percent(self) -> crate::percent::PercentDisplay
    where
        Self: crate::percent::PercentLike,
    {
        crate::percent::percent(self)
    }

    /// Formats this value as a human-readable percentage using custom options.
    fn human_percent_with(
        self,
        options: crate::percent::PercentOptions,
    ) -> crate::percent::PercentDisplay
    where
        Self: crate::percent::PercentLike,
    {
        crate::percent::percent_with(self, options)
    }
}

impl<T> Humanize for T {}
