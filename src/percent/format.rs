use core::fmt;
use core::fmt::Write;

use crate::common::fmt::{round_nonneg_f64, write_frac_digits, write_u128, POW10_F64};

use super::PercentOptions;

pub fn format_percent<W: fmt::Write>(
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
    let precision = options.precision as usize;
    let factor = POW10_F64[precision];

    // Round `abs` to `precision` decimal places using the selected mode.
    let rounded = round_nonneg_f64(abs, options.precision, options.rounding, negative);

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

/// Writes the fractional part of a rounded percentage value.
fn write_frac_part<W: fmt::Write>(
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
