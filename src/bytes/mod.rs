mod display;
mod format;
mod options;
mod traits;

pub use display::BytesDisplay;
pub use options::BytesOptions;
pub use traits::BytesLike;

/// Creates a human-readable byte-size formatter using default decimal units.
///
/// # Examples
///
/// ```rust
/// assert_eq!(humfmt::bytes(1536).to_string(), "1.5KB");
/// ```
pub fn bytes<T: BytesLike>(value: T) -> BytesDisplay {
    BytesDisplay::new(value.into_bytes(), BytesOptions::new())
}

/// Creates a human-readable byte-size formatter with custom options.
///
/// # Examples
///
/// ```rust
/// use humfmt::BytesOptions;
///
/// let out = humfmt::bytes_with(1536, BytesOptions::new().binary());
/// assert_eq!(out.to_string(), "1.5KiB");
/// ```
pub fn bytes_with<T: BytesLike>(value: T, options: BytesOptions) -> BytesDisplay {
    BytesDisplay::new(value.into_bytes(), options)
}
