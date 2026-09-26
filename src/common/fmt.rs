use core::cmp::Ordering;
use core::fmt;

use crate::rounding::carry_after_truncation;

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
        // Invariant: the buffer is only ever written via `fmt::Write::write_str`,
        // which only accepts valid UTF-8 `&str` input. The bytes therefore form
        // valid UTF-8 by construction.
        debug_assert!(core::str::from_utf8(&self.buf[..self.len]).is_ok());

        // SAFETY: see invariant above.
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

/// Counts the characters written to it.
///
/// Used to size a padded field: the formatter runs once into this sink to learn
/// how wide the rendered value is, then again into the real output. Counting is
/// character-based to match how `Formatter` measures padding.
#[derive(Default)]
pub(crate) struct CharCounter {
    chars: usize,
}

impl fmt::Write for CharCounter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.chars += s.chars().count();
        Ok(())
    }
}

/// Padding requested by a format specifier: width, fill character and alignment.
pub(crate) struct Pad {
    width: usize,
    fill: char,
    align: Option<fmt::Alignment>,
}

/// Renders a value into a writer of any kind.
///
/// The generic method lets one body run into both the width-counting sink inside
/// [`write_padded`] and the real output; a closure would be tied to one writer type.
pub(crate) trait Render {
    fn render<W: fmt::Write + ?Sized>(&self, f: &mut W) -> fmt::Result;
}

/// Renders a value, padded only when the specifier asks for a width.
///
/// The check lives here rather than in [`write_padded`], so the common unpadded
/// path calls the renderer directly instead of building padding settings.
#[inline]
pub(crate) fn fmt_with_padding<R>(body: &R, f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    R: Render + ?Sized,
{
    match f.width() {
        Some(width) if width > 0 => {
            let pad = Pad {
                width,
                fill: f.fill(),
                align: f.align(),
            };

            write_padded(pad, f, body)
        }
        _ => body.render(f),
    }
}

/// Writes the renderer output, padded to the requested width.
///
/// The value is rendered once into a character sink to learn how wide it is and
/// once into the output together with the padding.
///
/// Alignment follows `Formatter` conventions, including left as the default. The
/// precision is deliberately not used for truncation: cutting a formatted value
/// would drop its unit suffix (`"15.3K"` would become `"15"`).
pub(crate) fn write_padded<W, R>(pad: Pad, f: &mut W, body: &R) -> fmt::Result
where
    W: fmt::Write + ?Sized,
    R: Render + ?Sized,
{
    let mut counter = CharCounter::default();
    body.render(&mut counter)?;

    let width = pad.width.saturating_sub(counter.chars);

    if width == 0 {
        return body.render(f);
    }

    let (leading, trailing) = match pad.align {
        Some(fmt::Alignment::Right) => (width, 0),
        Some(fmt::Alignment::Center) => (width / 2, width - width / 2),
        _ => (0, width),
    };

    for _ in 0..leading {
        f.write_char(pad.fill)?;
    }

    body.render(f)?;

    for _ in 0..trailing {
        f.write_char(pad.fill)?;
    }

    Ok(())
}

/// Writes an ASCII digit string with grouping separators every 3 digits from the right.
///
/// Example with separator `','`: `"12345"` -> `"12,345"`.
pub(crate) fn write_grouped_ascii_digits<W: fmt::Write + ?Sized>(
    f: &mut W,
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

/// Decimal digits in a `u128` when written out.
const U128_DIGITS: usize = 39;

/// Largest trailing-zero run added by significant-digit rounding.
///
/// Rounding keeps at least one significant digit, so the shift is at most
/// `int_digits - 1`.
const MAX_ROUNDING_ZEROS: usize = U128_DIGITS - 1;

/// Digits a scaled integer can print: significant digits followed by zeros.
const MAX_SCALED_DIGITS: usize = U128_DIGITS + MAX_ROUNDING_ZEROS;

/// Decimal digits in a `u64` when written out.
const U64_DIGITS: usize = 20;

/// ASCII digits of `00`..`99`, so two digits are written per step.
const DIGIT_PAIRS: [u8; 200] = {
    let mut pairs = [0u8; 200];
    let mut value = 0usize;

    while value < 100 {
        pairs[value * 2] = b'0' + (value / 10) as u8;
        pairs[value * 2 + 1] = b'0' + (value % 10) as u8;
        value += 1;
    }

    pairs
};

/// Writes the decimal digits of `value` into `buf` and returns their count.
///
/// `buf` must hold at least [`U128_DIGITS`] bytes. Zero is written as a single `'0'`.
#[inline]
fn write_digits_into(buf: &mut [u8], value: u128) -> usize {
    debug_assert!(buf.len() >= U128_DIGITS);

    // Most values a formatter sees fit in a `u64`, where division is a single
    // machine instruction instead of the multi-word sequence `u128` needs.
    let len = if value <= u64::MAX as u128 {
        write_digits_u64(buf, value as u64)
    } else {
        write_digits_u128(buf, value)
    };

    buf[..len].reverse();

    debug_assert!(buf[..len].iter().all(|b| b.is_ascii_digit()));

    len
}

/// Writes the decimal digits of a `u64` into `buf`, least significant first.
fn write_digits_u64(buf: &mut [u8], mut value: u64) -> usize {
    debug_assert!(buf.len() >= U64_DIGITS);

    let mut len = 0usize;

    while value >= 100 {
        let pair = ((value % 100) * 2) as usize;
        buf[len] = DIGIT_PAIRS[pair + 1];
        buf[len + 1] = DIGIT_PAIRS[pair];
        len += 2;
        value /= 100;
    }

    if value >= 10 {
        let pair = (value * 2) as usize;
        buf[len] = DIGIT_PAIRS[pair + 1];
        buf[len + 1] = DIGIT_PAIRS[pair];
        return len + 2;
    }

    buf[len] = b'0' + value as u8;

    len + 1
}

/// Writes the decimal digits of a `u128` into `buf`, least significant first.
fn write_digits_u128(buf: &mut [u8], mut value: u128) -> usize {
    debug_assert!(buf.len() >= U128_DIGITS);

    let mut len = 0usize;

    while value != 0 {
        buf[len] = b'0' + (value % 10) as u8;
        len += 1;
        value /= 10;
    }

    if len == 0 {
        buf[0] = b'0';
        return 1;
    }

    len
}

/// Writes a `u128` without heap allocation, with optional digit grouping.
pub(crate) fn write_u128<W: fmt::Write + ?Sized>(
    f: &mut W,
    value: u128,
    group: bool,
    group_separator: char,
) -> fmt::Result {
    let mut buf = [0u8; U128_DIGITS];
    let len = write_digits_into(&mut buf, value);

    // SAFETY: bytes are ASCII '0'..='9' produced above, valid UTF-8.
    let digits = unsafe { core::str::from_utf8_unchecked(&buf[..len]) };

    if group {
        write_grouped_ascii_digits(f, digits, group_separator)
    } else {
        f.write_str(digits)
    }
}

/// Integer part of a scaled value, which can be wider than `u128`.
///
/// Significant-digit rounding can produce an integer that no `u128` holds:
/// `u128::MAX` rounded up to one significant digit is `4 * 10^38`. Keeping the
/// significant digits next to a trailing-zero count represents that exactly
/// without introducing a wider integer type.
#[derive(Copy, Clone, Debug)]
pub(crate) struct ScaledInteger {
    digits: u128,
    zeros: u8,
}

impl ScaledInteger {
    /// Creates a value with no trailing zeros.
    #[inline]
    pub(crate) const fn plain(digits: u128) -> Self {
        Self { digits, zeros: 0 }
    }

    /// Creates a value from significant digits followed by `zeros` zeros.
    #[inline]
    pub(crate) const fn with_zeros(digits: u128, zeros: u8) -> Self {
        debug_assert!(zeros as usize <= MAX_ROUNDING_ZEROS);
        Self { digits, zeros }
    }

    /// Returns `true` when the value is exactly zero.
    #[inline]
    pub(crate) fn is_zero(self) -> bool {
        self.digits == 0
    }

    /// Returns `true` when the value is exactly one.
    #[inline]
    pub(crate) fn is_one(self) -> bool {
        self.digits == 1 && self.zeros == 0
    }

    /// Returns `true` when the value is at least `threshold`.
    ///
    /// Compares against the threshold scaled down by the zero run, because the
    /// full value can exceed `u128`.
    #[inline]
    pub(crate) fn at_least(self, threshold: u128) -> bool {
        if self.zeros == 0 {
            return self.digits >= threshold;
        }

        let scale = 10u128.pow(self.zeros as u32);

        // Ceiling division: `digits * 10^zeros >= threshold` is equivalent to
        // `digits >= ceil(threshold / 10^zeros)`.
        self.digits >= (threshold + scale - 1) / scale
    }
}

/// Writes the integer part of a scaled value, honouring digit grouping.
pub(crate) fn write_scaled_integer<W: fmt::Write + ?Sized>(
    f: &mut W,
    integer: ScaledInteger,
    group: bool,
    group_separator: char,
) -> fmt::Result {
    if integer.zeros == 0 {
        return write_u128(f, integer.digits, group, group_separator);
    }

    // The zeros sit behind the significant digits, so that grouping can count
    // digit positions from the right of the whole number.
    let mut buf = [b'0'; MAX_SCALED_DIGITS];
    let len = write_digits_into(&mut buf, integer.digits);
    let total = len + integer.zeros as usize;

    // SAFETY: `buf[..total]` covers the digits written above plus the zero run,
    // whose length is bounded by `with_zeros`; every byte written is ASCII.
    let digits = unsafe { core::str::from_utf8_unchecked(&buf[..total]) };

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
    pub(crate) integer: ScaledInteger,
    pub(crate) frac_digits: [u8; 6],
    pub(crate) frac_len: u8,
}

impl DecimalParts {
    /// Returns `true` if the value is exactly `1` with no fractional part.
    /// Used for English singular/plural selection in byte labels.
    pub(crate) fn is_exactly_one(&self) -> bool {
        self.integer.is_one() && self.frac_len == 0
    }
}

/// Produces rounded decimal parts for `magnitude / unit` using the specified rounding mode.
///
/// Uses long division for fractional digits — safe for the full `u128` range
/// without any intermediate multiplication overflow.
pub(crate) fn decimal_parts_rounded(
    magnitude: u128,
    unit: u128,
    precision: u8,
    rounding: crate::RoundingMode,
    is_negative: bool,
) -> DecimalParts {
    let precision = precision.min(6);

    // Values that fit in a `u64` divide in one instruction; the `u128` path is
    // reserved for the top of the range, where the divisor may not fit either.
    let (mut integer, remainder) = if magnitude <= u64::MAX as u128 && unit <= u64::MAX as u128 {
        let magnitude = magnitude as u64;
        let unit = unit as u64;

        ((magnitude / unit) as u128, (magnitude % unit) as u128)
    } else {
        (magnitude / unit, magnitude % unit)
    };

    let (frac_digits, mut frac_len, carry) =
        fractional_digits_rounded(remainder, unit, precision, rounding, is_negative);

    if carry {
        integer = integer.saturating_add(1);
    }

    // Trim trailing zeros.
    while frac_len != 0 && frac_digits[(frac_len - 1) as usize] == b'0' {
        frac_len -= 1;
    }

    DecimalParts {
        integer: ScaledInteger::plain(integer),
        frac_digits,
        frac_len,
    }
}

fn fractional_digits_rounded(
    remainder: u128,
    unit: u128,
    precision: u8,
    rounding: crate::RoundingMode,
    is_negative: bool,
) -> ([u8; 6], u8, bool) {
    debug_assert!(unit != 0);
    debug_assert!(remainder < unit);

    let mut digits = [b'0'; 6];
    let mut rem = remainder;

    if precision == 0 {
        let has_remainder = rem > 0;
        let next_digit = if has_remainder {
            mul10_div_mod(rem, unit).0
        } else {
            0
        };
        let carry = carry_after_truncation(next_digit >= 5, has_remainder, rounding, is_negative);

        return (digits, 0, carry);
    }

    for slot in digits.iter_mut().take(precision as usize) {
        let (digit, next_rem) = mul10_div_mod(rem, unit);

        debug_assert!(digit <= 9);

        *slot = b'0' + digit;
        rem = next_rem;
    }

    let has_remainder = rem > 0;
    let next_digit = if has_remainder {
        mul10_div_mod(rem, unit).0
    } else {
        0
    };
    let carry = carry_after_truncation(next_digit >= 5, has_remainder, rounding, is_negative);

    if !carry {
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

/// Computes `(remainder * 10) / unit` and `(remainder * 10) % unit` without overflow.
///
/// `remainder < unit`, so the returned digit is always in `0..=9`. The common
/// path uses normal `u64` arithmetic; the wide fallback is only used for
/// extreme values near the top of the `u128` range.
#[inline]
fn mul10_div_mod(remainder: u128, unit: u128) -> (u8, u128) {
    debug_assert!(unit != 0);
    debug_assert!(remainder < unit);

    if remainder <= u64::MAX as u128 && unit <= u64::MAX as u128 {
        let (digit, rem) = mul10_div_mod_u64(remainder as u64, unit as u64);
        return (digit, rem as u128);
    }

    if remainder <= u128::MAX / 10 {
        let product = remainder * 10;
        return ((product / unit) as u8, product % unit);
    }

    mul10_div_mod_wide(remainder, unit)
}

/// Computes `(remainder * 10) / unit` and `(remainder * 10) % unit` for `u64` inputs.
///
/// `remainder < unit`, so the quotient is always a single digit.
#[inline]
fn mul10_div_mod_u64(remainder: u64, unit: u64) -> (u8, u64) {
    debug_assert!(unit != 0);
    debug_assert!(remainder < unit);

    if remainder <= u64::MAX / 10 {
        let product = remainder * 10;

        return ((product / unit) as u8, product % unit);
    }

    // `remainder * 10` overflows `u64`, so the digit is found by counting down
    // while comparing in `u128`, where `unit * digit` cannot overflow.
    let product = u128::from(remainder) * 10;
    let mut digit = 9u8;

    loop {
        let candidate = u128::from(unit) * u128::from(digit);

        if candidate <= product {
            return (digit, (product - candidate) as u64);
        }

        digit -= 1;
    }
}

fn mul10_div_mod_wide(remainder: u128, unit: u128) -> (u8, u128) {
    debug_assert!(unit != 0);
    debug_assert!(remainder < unit);
    debug_assert!(remainder > u128::MAX / 10);

    let (product_hi, product_lo) = mul_u128_by_u8_wide(remainder, 10);

    // Since `remainder < unit`, `(remainder * 10) / unit` is always in `0..=9`.
    for digit in (0u8..=9).rev() {
        let (candidate_hi, candidate_lo) = mul_u128_by_u8_wide(unit, digit);

        if cmp_wide(candidate_hi, candidate_lo, product_hi, product_lo) != Ordering::Greater {
            let (rem_hi, rem_lo) = sub_wide(product_hi, product_lo, candidate_hi, candidate_lo);

            debug_assert_eq!(rem_hi, 0);
            debug_assert!(rem_lo < unit);

            return (digit, rem_lo);
        }
    }

    unreachable!("digit 0 is always a valid quotient candidate")
}

#[inline]
fn mul_u128_by_u8_wide(value: u128, multiplier: u8) -> (u128, u128) {
    const MASK_64: u128 = u64::MAX as u128;

    let multiplier = multiplier as u128;

    let lo_part = (value & MASK_64) * multiplier;
    let carry = lo_part >> 64;
    let lo_low = lo_part & MASK_64;

    let hi_part = (value >> 64) * multiplier + carry;
    let hi = hi_part >> 64;
    let lo = ((hi_part & MASK_64) << 64) | lo_low;

    (hi, lo)
}

#[inline]
fn cmp_wide(a_hi: u128, a_lo: u128, b_hi: u128, b_lo: u128) -> Ordering {
    match a_hi.cmp(&b_hi) {
        Ordering::Equal => a_lo.cmp(&b_lo),
        other => other,
    }
}

#[inline]
fn sub_wide(a_hi: u128, a_lo: u128, b_hi: u128, b_lo: u128) -> (u128, u128) {
    debug_assert!(cmp_wide(b_hi, b_lo, a_hi, a_lo) != Ordering::Greater);

    let (lo, borrowed) = a_lo.overflowing_sub(b_lo);
    let hi = a_hi - b_hi - u128::from(borrowed);

    (hi, lo)
}

/// Computes significant digits for u128 magnitudes.
#[inline]
pub(crate) fn compute_sigfigs_u128(
    magnitude: u128,
    unit: u128,
    sig_figs: u8,
    rounding: crate::RoundingMode,
    negative: bool,
) -> (u8, DecimalParts) {
    if magnitude == 0 {
        return (
            sig_figs.saturating_sub(1),
            decimal_parts_rounded(0, unit, 0, rounding, negative),
        );
    }

    let scaled_int = magnitude / unit;
    let int_digits = if scaled_int == 0 {
        1
    } else {
        (scaled_int.ilog10() + 1) as u8
    };

    let shift = sig_figs as i32 - int_digits as i32;

    if shift >= 0 {
        let mut decimals = (shift as u8).min(6);
        let mut parts = decimal_parts_rounded(magnitude, unit, decimals, rounding, negative);

        let new_int_digits = if parts.integer.digits == 0 {
            1
        } else {
            (parts.integer.digits.ilog10() + 1) as u8
        };

        if new_int_digits > int_digits && decimals > 0 {
            decimals -= 1;

            if parts.frac_len > decimals {
                parts.frac_len = decimals;
            }
        }

        (decimals, parts)
    } else {
        let drop_digits = (-shift) as u32;
        let round_factor = 10u128.pow(drop_digits);

        // `unit * 10^drop_digits <= magnitude`, because `int_digits` counts the
        // digits of `magnitude / unit`; the product therefore fits in `u128`.
        debug_assert!(unit <= u128::MAX / round_factor);

        let new_unit = unit * round_factor;
        let mut parts = decimal_parts_rounded(magnitude, new_unit, 0, rounding, negative);

        // The scaled integer is `parts.integer * 10^drop_digits`, which can
        // exceed `u128`: `u128::MAX` rounded up to one significant digit is
        // `4 * 10^38`.
        parts.integer = ScaledInteger::with_zeros(parts.integer.digits, drop_digits as u8);

        (0, parts)
    }
}

/// Writes fractional digits (ASCII bytes) directly to the formatter.
pub(crate) fn write_frac_digits<W: fmt::Write + ?Sized>(f: &mut W, digits: &[u8]) -> fmt::Result {
    debug_assert!(digits.iter().all(|b| b.is_ascii_digit()));

    // SAFETY: digits are always ASCII bytes in '0'..='9', produced by
    // fractional_digits_rounded. They are valid UTF-8 by construction.
    let s = unsafe { core::str::from_utf8_unchecked(digits) };

    f.write_str(s)
}
