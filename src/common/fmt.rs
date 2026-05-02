use core::fmt;
use core::fmt::Write;

/// A tiny stack-backed string buffer used to avoid heap allocations during formatting.
///
/// Written to only via `fmt::Write::write_str`, which guarantees UTF-8 input,
/// so the buffer content is always valid UTF-8.
pub(crate) struct StackString<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> StackString<N> {
    pub(crate) const fn new() -> Self {
        Self {
            buf: [0u8; N],
            len: 0,
        }
    }

    #[inline]
    pub(crate) fn as_str(&self) -> &str {
        // Safety: the buffer is only written via `fmt::Write::write_str`, which
        // only accepts valid UTF-8 `&str` input. Therefore the resulting bytes
        // are always valid UTF-8.
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }
}

impl<const N: usize> Default for StackString<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Write for StackString<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        if self.len + bytes.len() > N {
            return Err(fmt::Error);
        }
        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }
}

/// Writes an ASCII digit string with grouping separators every 3 digits from the right.
///
/// Example with separator `','`: `"12345"` → `"12,345"`.
pub(crate) fn write_grouped_ascii_digits(
    f: &mut fmt::Formatter<'_>,
    digits: &str,
    group_separator: char,
) -> fmt::Result {
    let len = digits.len();
    if len <= 3 {
        return f.write_str(digits);
    }

    let first = match len % 3 {
        0 => 3,
        n => n,
    };

    f.write_str(&digits[..first])?;
    let mut pos = first;
    while pos < len {
        f.write_char(group_separator)?;
        f.write_str(&digits[pos..pos + 3])?;
        pos += 3;
    }

    Ok(())
}

/// Writes a `u128` without heap allocation, with optional digit grouping.
pub(crate) fn write_u128(
    f: &mut fmt::Formatter<'_>,
    mut value: u128,
    group: bool,
    group_separator: char,
) -> fmt::Result {
    if value == 0 {
        return f.write_str("0");
    }

    // u128::MAX is 39 decimal digits.
    let mut rev = [0u8; 39];
    let mut len = 0usize;

    while value != 0 {
        rev[len] = b'0' + (value % 10) as u8;
        len += 1;
        value /= 10;
    }

    let mut fwd = [0u8; 39];
    for i in 0..len {
        fwd[i] = rev[len - 1 - i];
    }

    let digits = unsafe { core::str::from_utf8_unchecked(&fwd[..len]) };

    if group {
        write_grouped_ascii_digits(f, digits, group_separator)
    } else {
        f.write_str(digits)
    }
}

/// A compact "integer + fractional digits" representation used for scaled outputs.
///
/// Fractional digits are stored as ASCII bytes. `frac_len` reflects the number
/// of significant digits after trimming trailing zeros.
#[derive(Copy, Clone, Debug)]
pub(crate) struct DecimalParts {
    pub(crate) integer: u128,
    pub(crate) frac_digits: [u8; 6],
    pub(crate) frac_len: u8,
}

impl DecimalParts {
    /// Returns `true` if the value is exactly `1` with no fractional part.
    /// Used for English singular/plural selection in byte labels.
    pub(crate) fn is_exactly_one(&self) -> bool {
        self.integer == 1 && self.frac_len == 0
    }

    /// Converts the parts to `f64` for locale suffix inflection hooks.
    ///
    /// Note: precision is limited to 6 decimal places and f64 cannot exactly
    /// represent all large integers, but for display purposes this is sufficient.
    pub(crate) fn as_f64(&self) -> f64 {
        let mut value = self.integer as f64;
        let mut denom = 10.0;
        for i in 0..(self.frac_len as usize) {
            value += (self.frac_digits[i] - b'0') as f64 / denom;
            denom *= 10.0;
        }
        value
    }
}

/// Produces rounded decimal parts for `magnitude / unit` using half-up rounding.
///
/// Uses long division for fractional digits — safe for the full `u128` range
/// without any intermediate multiplication overflow.
pub(crate) fn decimal_parts_rounded(magnitude: u128, unit: u128, precision: u8) -> DecimalParts {
    let precision = precision.min(6);
    let mut integer = magnitude / unit;
    let remainder = magnitude % unit;

    let (frac_digits, mut frac_len, carry) = fractional_digits_rounded(remainder, unit, precision);

    if carry {
        integer = integer.saturating_add(1);
    }

    // Trim trailing zeros.
    while frac_len != 0 && frac_digits[(frac_len - 1) as usize] == b'0' {
        frac_len -= 1;
    }

    DecimalParts {
        integer,
        frac_digits,
        frac_len,
    }
}

fn fractional_digits_rounded(remainder: u128, unit: u128, precision: u8) -> ([u8; 6], u8, bool) {
    let mut digits = [b'0'; 6];
    let mut rem = remainder;

    if precision == 0 {
        rem = rem.saturating_mul(10);
        let round_digit = rem / unit;
        return (digits, 0, round_digit >= 5);
    }

    for slot in digits.iter_mut().take(precision as usize) {
        rem = rem.saturating_mul(10);
        let digit = rem / unit;
        rem %= unit;
        *slot = b'0' + digit as u8;
    }

    // One digit beyond precision to decide rounding direction.
    rem = rem.saturating_mul(10);
    let round_digit = rem / unit;

    if round_digit < 5 {
        return (digits, precision, false);
    }

    // Propagate carry through fractional digits.
    let mut idx = precision as i32 - 1;
    while idx >= 0 {
        let i = idx as usize;
        if digits[i] != b'9' {
            digits[i] += 1;
            return (digits, precision, false);
        }
        digits[i] = b'0';
        idx -= 1;
    }

    // Carry propagated past all fractional digits — increment integer part.
    (digits, precision, true)
}

/// Writes fractional digits (ASCII bytes) directly to the formatter.
pub(crate) fn write_frac_digits(f: &mut fmt::Formatter<'_>, digits: &[u8]) -> fmt::Result {
    // Safety: digits are always ASCII bytes in '0'..='9', produced by
    // fractional_digits_rounded. They are valid UTF-8 by construction.
    let s = unsafe { core::str::from_utf8_unchecked(digits) };
    f.write_str(s)
}
