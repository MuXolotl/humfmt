#![doc = include_str!("../docs/CRATE.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]

pub mod ago;
pub mod bytes;
#[cfg(feature = "chrono")]
#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
pub mod chrono;
pub mod duration;
mod error;
pub mod list;
pub mod locale;
pub mod number;
pub mod ordinal;
pub mod prelude;
#[cfg(feature = "time")]
#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
pub mod time;

mod common;
mod traits;

pub use ago::{ago, ago_with, AgoDisplay};
pub use bytes::{bytes, bytes_with, BytesDisplay, BytesLike, BytesOptions};
pub use duration::{duration, duration_with, DurationDisplay, DurationLike, DurationOptions};
pub use error::{DurationConversionError, NegativeDurationError};
pub use list::{list, list_with, ListDisplay, ListOptions};
pub use number::{number, number_with, NumberDisplay, NumberOptions};
pub use ordinal::{ordinal, ordinal_with, OrdinalDisplay, OrdinalLike};
pub use traits::Humanize;
