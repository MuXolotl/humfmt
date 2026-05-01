use core::fmt;
use core::fmt::Write;

use crate::common::fmt::{
    decimal_parts_rounded, trim_ascii_trailing_zeros_and_dot, write_frac_digits, write_u128,
    StackString,
};
use crate::common::numeric::NumericValue;

use super::NumberOptions;

const POW1000: [u128; 12] = [
    1,
    1_000,
    1_000_000,
    1_000_000_000,
    1_000_000_000_000,
    1_000_000_000_000_000,
    1_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000_000_000,
    1_000_000_000_000_000_000_000_000_000_000_000,
];

// Powers of 1000 as f64, used for O(1) float scaling without std float methods.
const POW1000_F64: [f64; 12] = [
    1.0,
    1_000.0,
    1_000_000.0,
    1_000_000_000.0,
    1_000_000_000_000.0,
    1_000_000_000_000_000.0,
    1_000_000_000_000_000_000.0,
    1_000_000_000_000_000_000_000.0,
    1_000_000_000_000_000_000_000_000.0,
    1_000_000_000_000_000_000_000_000_000.0,
    1_000_000_000_000_000_000_000_000_000_000.0,
    1_000_000_000_000_000_000_000_000_000_000_000.0,
];

const POW10_F64: [f64; 7] = [1.0, 10.0, 100.0, 1_000.0, 10_000.0, 100_000.0, 1_000_000.0];

pub fn format_number<L: crate::locale::Locale>(
    f: &mut fmt::Formatter<'_>,
    value: NumericValue,
    options: &NumberOptions<L>,
) -> fmt::Result {
    match value {
        NumericValue::Int(v) => format_int(f, v, options),
        NumericValue::UInt(v) => format_uint(f, v, options),
        NumericValue::Float(v) => format_float(f, v, options),
    }
}

fn format_int<L: crate::locale::Locale>(
    f: &mut fmt::Formatter<'_>,
    value: i128,
    options: &NumberOptions<L>,
) -> fmt::Result {
    let negative = value.is_negative();
    let magnitude = if negative {
        value.unsigned_abs()
    } else {
        value as u128
    };

    format_u128_magnitude(f, negative && magnitude != 0, magnitude, options)
}

fn format_uint<L: crate::locale::Locale>(
    f: &mut fmt::Formatter<'_>,
    value: u128,
    options: &NumberOptions<L>,
) -> fmt::Result {
    format_u128_magnitude(f, false, value, options)
}

fn format_u128_magnitude<L: crate::locale::Locale>(
    f: &mut fmt::Formatter<'_>,
    negative: bool,
    magnitude: u128,
    options: &NumberOptions<L>,
) -> fmt::Result {
    let locale = &options.locale;
    let precision = options.precision;
    let max_idx = locale.max_compact_suffix_index().min(POW1000.len() - 1);

    let (mut idx, unit) = compute_compact_unit(magnitude, max_idx);
    let mut parts = decimal_parts_rounded(magnitude, unit, precision);

    if parts.integer >= 1_000 && idx < max_idx {
        idx += 1;
        parts = decimal_parts_rounded(magnitude, POW1000[idx], precision);
    }

    if negative {
        f.write_str("-")?;
    }

    write_u128(
        f,
        parts.integer,
        options.separators && idx == 0,
        locale.group_separator(),
    )?;

    if options.fixed_precision {
        if precision > 0 {
            f.write_char(locale.decimal_separator())?;
            let existing = parts.frac_len as usize;
            write_frac_digits(f, &parts.frac_digits[..existing])?;
            for _ in existing..precision as usize {
                f.write_char('0')?;
            }
        }
    } else if parts.frac_len != 0 {
        f.write_char(locale.decimal_separator())?;
        write_frac_digits(f, &parts.frac_digits[..parts.frac_len as usize])?;
    }

    let suffix = locale.compact_suffix_for(idx, parts.as_f64(), options.long_units);
    f.write_str(suffix)
}

#[inline]
fn compute_compact_unit(magnitude: u128, max_idx: usize) -> (usize, u128) {
    if magnitude < 1_000 || max_idx == 0 {
        return (0, 1);
    }

    let idx = ((magnitude.ilog10() / 3) as usize).min(max_idx);
    (idx, POW1000[idx])
}

fn format_float<L: crate::locale::Locale>(
    f: &mut fmt::Formatter<'_>,
    raw: f64,
    options: &NumberOptions<L>,
) -> fmt::Result {
    if !raw.is_finite() {
        return write!(f, "{raw}");
    }

    let locale = &options.locale;
    let precision = options.precision;
    let max_idx = locale.max_compact_suffix_index();

    let abs = raw.abs();
    let (scaled, idx) = normalize_scaled_o1(abs, precision, max_idx);

    let negative = raw.is_sign_negative() && scaled != 0.0;
    if negative {
        f.write_str("-")?;
    }

    // Write the scaled value into a stack buffer so we can apply locale-aware
    // decimal separator and trailing-zero trimming without heap allocation.
    // The buffer is 64 bytes which is always sufficient for precision <= 6
    // plus a small integer part (e.g. "999.123456" is 10 chars).
    let mut tmp = StackString::<64>::new();
    let write_ok = write!(&mut tmp, "{:.*}", precision as usize, scaled).is_ok();

    if write_ok {
        if !options.fixed_precision {
            trim_ascii_trailing_zeros_and_dot(&mut tmp);
        }

        write_localized_numeric_str(
            f,
            tmp.as_str(),
            options.separators,
            locale.decimal_separator(),
            locale.group_separator(),
        )?;
    } else {
        // Fallback: the stack buffer overflowed (should not happen for precision <= 6,
        // but we handle it defensively). We write directly with the locale decimal
        // separator by splitting on '.'.
        write_float_direct(f, scaled, precision, options.fixed_precision, locale)?;
    }

    let suffix = locale.compact_suffix_for(idx, scaled, options.long_units);
    f.write_str(suffix)
}

/// Fallback float writer used when the stack buffer overflows.
///
/// Writes `value` with `precision` decimal places, applying the locale decimal
/// separator. This path avoids StackString but still respects the locale.
fn write_float_direct<L: crate::locale::Locale>(
    f: &mut fmt::Formatter<'_>,
    value: f64,
    precision: u8,
    fixed_precision: bool,
    locale: &L,
) -> fmt::Result {
    // Decompose into integer and fractional parts using integer arithmetic
    // to stay no_std compatible and avoid a second stack buffer.
    let p = precision as usize;
    let factor = POW10_F64[p.min(POW10_F64.len() - 1)];
    let shifted = (value * factor + 0.5) as u64;
    let int_part = shifted / factor as u64;
    let frac_raw = shifted % factor as u64;

    write_u128(f, int_part as u128, false, locale.group_separator())?;

    if fixed_precision && precision > 0 {
        f.write_char(locale.decimal_separator())?;
        // Write fractional digits with leading zeros preserved.
        write_padded_frac(f, frac_raw, precision)?;
    } else if !fixed_precision {
        // Trim trailing zeros from the fractional part.
        if frac_raw != 0 {
            f.write_char(locale.decimal_separator())?;
            write_padded_frac_trimmed(f, frac_raw, precision)?;
        }
    }

    Ok(())
}

/// Writes a fractional value as exactly `precision` digits, zero-padded on the left.
fn write_padded_frac(f: &mut fmt::Formatter<'_>, frac: u64, precision: u8) -> fmt::Result {
    let p = precision as usize;
    let mut buf = [b'0'; 6];
    let mut rem = frac;
    for i in (0..p).rev() {
        buf[i] = b'0' + (rem % 10) as u8;
        rem /= 10;
    }
    let s = unsafe { core::str::from_utf8_unchecked(&buf[..p]) };
    f.write_str(s)
}

/// Writes a fractional value with trailing zeros trimmed.
fn write_padded_frac_trimmed(f: &mut fmt::Formatter<'_>, frac: u64, precision: u8) -> fmt::Result {
    let p = precision as usize;
    let mut buf = [b'0'; 6];
    let mut rem = frac;
    for i in (0..p).rev() {
        buf[i] = b'0' + (rem % 10) as u8;
        rem /= 10;
    }
    let mut end = p;
    while end > 0 && buf[end - 1] == b'0' {
        end -= 1;
    }
    if end > 0 {
        let s = unsafe { core::str::from_utf8_unchecked(&buf[..end]) };
        f.write_str(s)?;
    }
    Ok(())
}

/// Computes the compact scale index and scaled value for a non-negative finite f64.
///
/// Uses the IEEE 754 binary exponent to estimate the base-1000 index in O(1),
/// then adjusts by at most one step to correct for floating-point imprecision.
/// This matches the O(1) approach used for integers via `ilog10`.
///
/// The exponent of an f64 is stored in bits 52..=62 with a bias of 1023.
/// Dividing the unbiased exponent by log2(1000) ≈ 9.966 gives the approximate
/// power-of-1000 index. Integer arithmetic is used throughout to stay
/// compatible with `no_std` stable Rust.
fn normalize_scaled_o1(value: f64, precision: u8, max_idx: usize) -> (f64, usize) {
    if value < 1_000.0 || max_idx == 0 {
        let scaled = round_f64(value, precision);
        return if scaled >= 1_000.0 && max_idx > 0 {
            (scaled / 1_000.0, 1)
        } else {
            (scaled, 0)
        };
    }

    // Extract the unbiased binary exponent from the IEEE 754 bit representation.
    // For a normal f64: exponent bits are [52..62], bias is 1023.
    // log2(1000) ≈ 9.9658, so floor(unbiased_exp / 9.966) approximates
    // the base-1000 index. Integer fraction 1000/9966 avoids any f64 math here.
    let bits = value.to_bits();
    let biased_exp = ((bits >> 52) & 0x7FF) as i32;
    let unbiased_exp = biased_exp - 1023;

    let approx_idx = if unbiased_exp <= 0 {
        0usize
    } else {
        ((unbiased_exp as usize) * 1_000 / 9_966).min(max_idx)
    };

    // The approximation can be off by one. Correct downward if the value is
    // actually below the threshold for approx_idx.
    let idx = if approx_idx > 0 && approx_idx < POW1000_F64.len() && value < POW1000_F64[approx_idx]
    {
        approx_idx - 1
    } else {
        approx_idx
    };
    let idx = idx.min(max_idx);

    let divisor = if idx < POW1000_F64.len() {
        POW1000_F64[idx]
    } else {
        POW1000_F64[POW1000_F64.len() - 1]
    };

    let scaled = round_f64(value / divisor, precision);

    if scaled >= 1_000.0 && idx < max_idx {
        (scaled / 1_000.0, idx + 1)
    } else {
        (scaled, idx)
    }
}

/// Rounds a non-negative finite f64 to `precision` decimal places using half-up rounding.
///
/// Uses integer cast rounding instead of `f64::round()` because `round()` is not
/// available in `core` on stable Rust at MSRV 1.70 without `std` or `libm`.
/// The cast `(x + 0.5) as u64` produces correct half-up rounding for non-negative
/// finite values within the safe range.
#[inline]
fn round_f64(value: f64, precision: u8) -> f64 {
    if !value.is_finite() || value < 0.0 {
        return value;
    }

    let p = precision.min(6) as usize;
    let factor = POW10_F64[p];

    // Guard against overflow before shifting into integer range.
    let max_safe = f64::MAX / factor;
    if value > max_safe {
        return value;
    }

    let shifted = value * factor + 0.5;

    // Guard against values that exceed u64 range after shifting.
    if shifted >= u64::MAX as f64 {
        return value;
    }

    (shifted as u64) as f64 / factor
}

fn write_localized_numeric_str(
    f: &mut fmt::Formatter<'_>,
    input: &str,
    separators: bool,
    decimal_separator: char,
    group_separator: char,
) -> fmt::Result {
    // Split off any exponent part (e.g. "1.5e10") before processing.
    let (mantissa, exp_part) = match input
        .as_bytes()
        .iter()
        .position(|b| *b == b'e' || *b == b'E')
    {
        Some(pos) => (&input[..pos], Some(&input[pos..])),
        None => (input, None),
    };

    let (int_part, frac_part) = match mantissa.split_once('.') {
        Some((a, b)) => (a, Some(b)),
        None => (mantissa, None),
    };

    let int_is_digits = int_part.as_bytes().iter().all(|b| b.is_ascii_digit());

    if separators && int_is_digits {
        crate::common::fmt::write_grouped_ascii_digits(f, int_part, group_separator)?;
    } else {
        f.write_str(int_part)?;
    }

    if let Some(frac) = frac_part {
        f.write_char(decimal_separator)?;
        f.write_str(frac)?;
    }

    if let Some(exp) = exp_part {
        f.write_str(exp)?;
    }

    Ok(())
}
