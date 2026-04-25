pub trait Humanize: Sized {
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
}

impl<T> Humanize for T {}
