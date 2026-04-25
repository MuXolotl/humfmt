pub trait Humanize: Sized {
    fn human_duration(self) -> crate::duration::DurationDisplay
    where
        Self: crate::duration::DurationLike,
    {
        crate::duration::duration(self)
    }

    fn human_duration_with(
        self,
        options: crate::duration::DurationOptions,
    ) -> crate::duration::DurationDisplay
    where
        Self: crate::duration::DurationLike,
    {
        crate::duration::duration_with(self, options)
    }

    fn human_number(self) -> crate::number::NumberDisplay<crate::locale::English>
    where
        Self: crate::number::NumberLike,
    {
        crate::number::number(self)
    }

    fn human_number_with<L: crate::locale::Locale>(
        self,
        options: crate::number::NumberOptions<L>,
    ) -> crate::number::NumberDisplay<L>
    where
        Self: crate::number::NumberLike,
    {
        crate::number::number_with(self, options)
    }

    fn human_ordinal(self) -> crate::ordinal::OrdinalDisplay<crate::locale::English>
    where
        Self: crate::ordinal::OrdinalLike,
    {
        crate::ordinal::ordinal(self)
    }

    fn human_ordinal_with<L: crate::locale::Locale>(
        self,
        locale: L,
    ) -> crate::ordinal::OrdinalDisplay<L>
    where
        Self: crate::ordinal::OrdinalLike,
    {
        crate::ordinal::ordinal_with(self, locale)
    }
}

impl<T> Humanize for T {}
