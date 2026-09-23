use core::fmt;

use crate::common::fmt::{write_frac_digits, write_u128};
use crate::rounding::{decimal_factor, round_to_decimals};

use super::PercentOptions;

/// `u128::MAX` as `f64`: the smallest scaled magnitude the integer path cannot
/// write, because such a value no longer fits in a `u128`.
const U128_MAX_F64: f64 = u128::MAX as f64;

pub fn format_percent<W: fmt::Write + ?Sized>(
    f: &mut W,
    value: f64,
    options: &PercentOptions,
) -> fmt::Result {
    if !value.is_finite() {
        return write!(f, "{value}%");
    }

    let percent = value * 100.0;
    let negative = percent.is_sign_negative();
    let abs = percent.abs();

    // The product overflows to infinity for the largest ratios, and products at
    // or above `u128::MAX` cannot be written as an integer.
    if abs >= U128_MAX_F64 {
        return write_unbounded_percent(f, value, negative, options);
    }

    let precision = options.precision as usize;
    let factor = decimal_factor(options.precision);

    // Round `abs` to `precision` decimal places using the selected mode.
    let rounded = round_to_decimals(abs, options.precision, options.rounding, negative);

    let is_zero = rounded == 0.0;
    if negative && !is_zero {
        f.write_char('-')?;
    } else if options.force_sign && !negative && !is_zero {
        f.write_char('+')?;
    }

    let int_part = rounded as u128;
    write_u128(f, int_part, false, ',')?;

    if precision > 0 {
        write_frac_part(f, rounded, int_part, factor, precision, options)?;
    }

    f.write_char('%')
}

/// Writes percentages whose scaled magnitude does not fit in a `u128`.
///
/// The product is at or above `u128::MAX` for these ratios, and overflows to
/// infinity for the largest ones. Either way the magnitude is far beyond what
/// `f64` can hold fractionally, so the exact decimal expansion of `value` only
/// needs two zeros appended for the `* 100` step, and rounding cannot change
/// the result because there is nothing left to round away.
fn write_unbounded_percent<W: fmt::Write + ?Sized>(
    f: &mut W,
    value: f64,
    negative: bool,
    options: &PercentOptions,
) -> fmt::Result {
    if negative {
        f.write_char('-')?;
    } else if options.force_sign {
        f.write_char('+')?;
    }

    write!(f, "{:.0}", value.abs())?;
    f.write_str("00")?;

    if options.fixed_precision && options.precision > 0 {
        f.write_char(options.decimal_separator)?;

        for _ in 0..options.precision {
            f.write_char('0')?;
        }
    }

    f.write_char('%')
}

/// Writes the fractional part of a rounded percentage value.
fn write_frac_part<W: fmt::Write + ?Sized>(
    f: &mut W,
    rounded: f64,
    int_part: u128,
    factor: f64,
    precision: usize,
    options: &PercentOptions,
) -> fmt::Result {
    let frac_f = rounded - int_part as f64;

    // Shift the fractional portion and extract digits.
    let frac_shifted = (frac_f * factor + 0.5) as u64;
    // Clamp to prevent carry overflow (e.g. 0.999... * 10 + 0.5 = 10).
    let frac_clamped = frac_shifted.min(factor as u64 - 1);

    let mut buf = [b'0'; 6];
    let mut rem = frac_clamped;
    for i in (0..precision).rev() {
        buf[i] = b'0' + (rem % 10) as u8;
        rem /= 10;
    }

    if options.fixed_precision {
        f.write_char(options.decimal_separator)?;
        write_frac_digits(f, &buf[..precision])?;
    } else {
        let end = trim_trailing_zeros(&buf, precision);
        if end > 0 {
            f.write_char(options.decimal_separator)?;
            write_frac_digits(f, &buf[..end])?;
        }
    }

    Ok(())
}

#[inline]
fn trim_trailing_zeros(buf: &[u8; 6], len: usize) -> usize {
    let mut end = len;
    while end > 0 && buf[end - 1] == b'0' {
        end -= 1;
    }
    end
}
