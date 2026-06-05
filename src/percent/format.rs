use core::fmt;
use core::fmt::Write;

use crate::common::fmt::{write_frac_digits, write_u128};
use crate::RoundingMode;

use super::PercentOptions;

// Lookup table: 10^i for i in 0..=6, used to shift fractional digits.
const POW10: [f64; 7] = [1.0, 10.0, 100.0, 1_000.0, 10_000.0, 100_000.0, 1_000_000.0];

pub fn format_percent(
    f: &mut fmt::Formatter<'_>,
    value: f64,
    options: &PercentOptions,
) -> fmt::Result {
    if !value.is_finite() {
        return write!(f, "{value}%");
    }

    let percent = value * 100.0;
    let negative = percent.is_sign_negative();
    let abs = percent.abs();
    let precision = options.precision as usize;
    let factor = POW10[precision];

    // Round `abs` to `precision` decimal places using the selected mode.
    let rounded = round_percent(abs, factor, options.rounding, negative);

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

/// Rounds `abs` (already multiplied by 100) to `precision` decimal places.
///
/// Uses the same rounding semantics as the number and bytes formatters.
fn round_percent(abs: f64, factor: f64, rounding: RoundingMode, is_negative: bool) -> f64 {
    // Overflow guard: values near f64::MAX * factor would wrap u64.
    if abs * factor > u64::MAX as f64 {
        return abs;
    }

    let shifted = abs * factor;
    let trunc = shifted as u64;

    let has_remainder = shifted > trunc as f64;

    let carry = match rounding {
        RoundingMode::HalfUp => {
            // Ties round away from zero: 0.5 -> 1, -0.5 -> -1.
            (shifted + 0.5) as u64 > trunc
        }
        RoundingMode::Floor => is_negative && has_remainder,
        RoundingMode::Ceil => !is_negative && has_remainder,
    };

    let rounded_int = if carry { trunc + 1 } else { trunc };
    rounded_int as f64 / factor
}

/// Writes the fractional part of a rounded percentage value.
fn write_frac_part(
    f: &mut fmt::Formatter<'_>,
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
