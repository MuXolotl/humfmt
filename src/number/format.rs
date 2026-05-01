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

    let mut tmp = StackString::<64>::new();
    let write_res = write!(&mut tmp, "{:.*}", precision as usize, scaled);

    if write_res.is_err() {
        write!(f, "{:.*}", precision as usize, scaled)?;
        let suffix = locale.compact_suffix_for(idx, scaled, options.long_units);
        return f.write_str(suffix);
    }

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

    let suffix = locale.compact_suffix_for(idx, scaled, options.long_units);
    f.write_str(suffix)
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
/// compatible with `no_std` stable Rust (no `f64::log10`, no `f64::powi`).
fn normalize_scaled_o1(value: f64, precision: u8, max_idx: usize) -> (f64, usize) {
    if value < 1_000.0 || max_idx == 0 {
        let scaled = round_to_non_negative(value, precision);
        return if scaled >= 1_000.0 && max_idx > 0 {
            (scaled / 1_000.0, 1)
        } else {
            (scaled, 0)
        };
    }

    // Extract the unbiased binary exponent from the IEEE 754 representation.
    // For a normal f64: exponent bits are [52..62], bias is 1023.
    // log2(1000) ≈ 9.9658, so floor(unbiased_exp / 9.966) approximates
    // the base-1000 index. We use the integer fraction 1000/9966 to avoid
    // any floating-point math here, keeping this no_std compatible.
    let bits = value.to_bits();
    let biased_exp = ((bits >> 52) & 0x7FF) as i32;
    let unbiased_exp = biased_exp - 1023;

    let approx_idx = if unbiased_exp <= 0 {
        0usize
    } else {
        ((unbiased_exp as usize) * 1_000 / 9_966).min(max_idx)
    };

    // The approximation can be off by one due to the non-integer log2(1000) ratio.
    // Correct downward if the value is actually below the threshold for approx_idx.
    // POW1000_F64 is used instead of powi() to stay no_std compatible.
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

    let scaled = round_to_non_negative(value / divisor, precision);

    // After rounding, the scaled value may have crossed the 1000 boundary.
    if scaled >= 1_000.0 && idx < max_idx {
        (scaled / 1_000.0, idx + 1)
    } else {
        (scaled, idx)
    }
}

#[inline]
fn round_to_non_negative(value: f64, precision: u8) -> f64 {
    // `core` on stable does not expose `f64::round()` in no_std builds (MSRV 1.70).
    // Half-up rounding via integer cast is safe for non-negative finite values
    // within the representable u128 range.
    let p = (precision.min(6)) as usize;
    let factor = POW10_F64[p];

    if !value.is_finite() || value < 0.0 {
        return value;
    }

    let max_safe = (u128::MAX as f64) / factor;
    if value > max_safe {
        return value;
    }

    let scaled = (value * factor) + 0.5;
    ((scaled as u128) as f64) / factor
}

fn write_localized_numeric_str(
    f: &mut fmt::Formatter<'_>,
    input: &str,
    separators: bool,
    decimal_separator: char,
    group_separator: char,
) -> fmt::Result {
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
