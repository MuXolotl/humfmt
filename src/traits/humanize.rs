pub trait Humanize: Sized {
    fn human_number(self) -> crate::number::NumberDisplay<crate::locale::English>
    where
        Self: crate::number::NumberLike,
    {
        crate::number::number(self)
    }
}

impl<T> Humanize for T {}
