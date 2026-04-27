use core::fmt;
use core::fmt::Write;

/// A tiny stack-backed string buffer used to avoid heap allocations during formatting.
///
/// This is intentionally minimal and only supports ASCII output reliably.
/// It's used internally for float formatting and small intermediate renderings.
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

    pub(crate) fn as_str(&self) -> &str {
        // The buffer only ever receives valid UTF-8 via `fmt::Write` (`&str` input).
        core::str::from_utf8(&self.buf[..self.len]).unwrap_or("")
    }

    fn truncate(&mut self, new_len: usize) {
        self.len = new_len.min(self.len);
    }

    fn ends_with_byte(&self, byte: u8) -> bool {
        self.len != 0 && self.buf[self.len - 1] == byte
    }

    fn find_byte(&self, byte: u8) -> Option<usize> {
        self.buf[..self.len].iter().position(|b| *b == byte)
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

        let dst = &mut self.buf[self.len..self.len + bytes.len()];
        dst.copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }
}

/// Trims trailing `0` digits after a `.` and then removes the `.` if it becomes the last character.
/// This matches the "15.0K" -> "15K" behavior used throughout the crate.
pub(crate) fn trim_ascii_trailing_zeros_and_dot<const N: usize>(s: &mut StackString<N>) {
    let dot = match s.find_byte(b'.') {
        Some(pos) => pos,
        None => return,
    };

    // Trim trailing zeros, but never trim past the dot.
    while s.len > dot + 1 && s.ends_with_byte(b'0') {
        s.truncate(s.len - 1);
    }

    // If we trimmed everything after the dot, also remove the dot.
    if s.ends_with_byte(b'.') {
        s.truncate(s.len - 1);
    }
}

/// Writes an ASCII digit slice with grouping separators every 3 digits from the right.
///
/// Example (separator = '_'):
/// "12345" -> "12_345"
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

/// Writes a `u128` without allocations, optionally using digit grouping.
pub(crate) fn write_u128(
    f: &mut fmt::Formatter<'_>,
    mut value: u128,
    group: bool,
    group_separator: char,
) -> fmt::Result {
    if value == 0 {
        return f.write_str("0");
    }

    // u128 max is 39 decimal digits.
    let mut rev = [0u8; 39];
    let mut len = 0usize;

    while value != 0 {
        let digit = (value % 10) as u8;
        rev[len] = b'0' + digit;
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
/// Fractional digits are stored as ASCII bytes and may be shorter than `precision` after trimming.
#[derive(Copy, Clone, Debug)]
pub(crate) struct DecimalParts {
    pub(crate) integer: u128,
    pub(crate) frac_digits: [u8; 6],
    pub(crate) frac_len: u8,
}

impl DecimalParts {
    pub(crate) fn is_exactly_one(&self) -> bool {
        self.integer == 1 && self.frac_len == 0
    }

    pub(crate) fn as_f64(&self) -> f64 {
        let mut value = self.integer as f64;
        let mut denom = 10.0;

        for i in 0..(self.frac_len as usize) {
            let digit = (self.frac_digits[i] - b'0') as f64;
            value += digit / denom;
            denom *= 10.0;
        }

        value
    }
}

/// Produces rounded decimal parts for `magnitude / unit`, using "half-up" rounding.
/// `precision` is clamped to 6.
///
/// This function is designed to be safe for very large `u128` values:
/// it uses long division for fractional digits (no `remainder * 10^precision` multiplication).
pub(crate) fn decimal_parts_rounded(magnitude: u128, unit: u128, precision: u8) -> DecimalParts {
    let precision = precision.min(6);
    let mut integer = magnitude / unit;
    let remainder = magnitude % unit;

    let (frac_digits, mut frac_len, carry) = fractional_digits_rounded(remainder, unit, precision);

    if carry {
        integer = integer.saturating_add(1);
    }

    // Trim trailing zeros in fractional digits.
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
        // Round based on the first digit after decimal.
        rem = rem.saturating_mul(10);
        let round_digit = rem / unit;
        return (digits, 0, round_digit >= 5);
    }

    for slot in digits.iter_mut().take(precision as usize) {
        rem = rem.saturating_mul(10);
        let digit = rem / unit;
        rem %= unit;
        *slot = b'0' + (digit as u8);
    }

    // Look one digit ahead to decide rounding.
    rem = rem.saturating_mul(10);
    let round_digit = rem / unit;

    if round_digit < 5 {
        return (digits, precision, false);
    }

    // Increment the last digit with carry propagation.
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

    // Carry out of fractional digits increments the integer part.
    (digits, precision, true)
}

/// Writes fractional digits (ASCII bytes) to the formatter.
pub(crate) fn write_frac_digits(f: &mut fmt::Formatter<'_>, digits: &[u8]) -> fmt::Result {
    let s = unsafe { core::str::from_utf8_unchecked(digits) };
    f.write_str(s)
}
