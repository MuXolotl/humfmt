/// Private sealing infrastructure.
///
/// The `Sealed` trait is intentionally kept in a private module (`common` is not
/// public). Public extension traits in this crate use `Sealed` as a supertrait
/// to prevent downstream crates from implementing them for arbitrary types.
///
/// Only the types listed in this file can implement the public `*Like` traits.
pub trait Sealed {}

macro_rules! impl_sealed {
    ($($t:ty),* $(,)?) => {
        $(
            impl Sealed for $t {}
        )*
    };
}

impl_sealed!(
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    core::time::Duration,
);
