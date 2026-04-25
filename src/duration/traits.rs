use crate::common::sealed::Sealed;

pub trait DurationLike: Sealed + Copy {
    fn into_duration(self) -> core::time::Duration;
}

impl Sealed for core::time::Duration {}

impl DurationLike for core::time::Duration {
    fn into_duration(self) -> core::time::Duration {
        self
    }
}
