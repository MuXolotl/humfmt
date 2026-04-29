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
    let locale = options.locale_ref();
    let precision = options.precision_value();
    let max_idx = locale.max_compact_suffix_index().min(POW1000.len() - 1);

    let (mut idx, unit) = compute_compact_unit(magnitude, max_idx);
    let mut parts = decimal_parts_rounded(magnitude, unit, precision);

    // Rescale if rounding pushes us over the compact boundary (e.g. 999.95K -> 1M).
    if parts.integer >= 1_000 && idx < max_idx {
        idx += 1;
        parts = decimal_parts_rounded(magnitude, POW1000[idx], precision);
    }

    if negative {
        f.write_str("-")?;
    }

    // For idx > 0, the integer part is < 1000, so grouping separators won't matter.
    // For idx == 0, grouping can be useful if enabled.
    write_u128(
        f,
        parts.integer,
        options.separators_value() && idx == 0,
        locale.group_separator(),
    )?;

    if parts.frac_len != 0 {
        f.write_char(locale.decimal_separator())?;
        write_frac_digits(f, &parts.frac_digits[..parts.frac_len as usize])?;
    }

    let suffix = locale.compact_suffix_for(idx, parts.as_f64(), options.long_units_value());
    f.write_str(suffix)
}

#[inline]
fn compute_compact_unit(magnitude: u128, max_idx: usize) -> (usize, u128) {
    if magnitude < 1_000 || max_idx == 0 {
        return (0, 1);
    }

    // O(1) magnitude detection using base-10 integer logarithms.
    // Note: `ilog10` is undefined for zero; magnitude < 1000 includes zero.
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

    let locale = options.locale_ref();
    let precision = options.precision_value();
    let max_idx = locale.max_compact_suffix_index();

    let abs = raw.abs();
    let (scaled, idx) = normalize_scaled(abs, precision, max_idx);

    // Avoid "-0" after rounding tiny negatives.
    let negative = raw.is_sign_negative() && scaled != 0.0;
    if negative {
        f.write_str("-")?;
    }

    // Render the scaled number to a small stack buffer first, then localize it.
    // This is usually short (e.g. "15.3"), so a small buffer keeps stack usage low.
    let mut tmp = StackString::<64>::new();
    let write_res = write!(&mut tmp, "{:.*}", precision as usize, scaled);

    // If formatting into the fixed buffer fails (unexpectedly large float rendering),
    // fall back to writing directly without localization/separators.
    if write_res.is_err() {
        write!(f, "{:.*}", precision as usize, scaled)?;
        let suffix = locale.compact_suffix_for(idx, scaled, options.long_units_value());
        return f.write_str(suffix);
    }

    trim_ascii_trailing_zeros_and_dot(&mut tmp);

    write_localized_numeric_str(
        f,
        tmp.as_str(),
        options.separators_value(),
        locale.decimal_separator(),
        locale.group_separator(),
    )?;

    let suffix = locale.compact_suffix_for(idx, scaled, options.long_units_value());
    f.write_str(suffix)
}

fn normalize_scaled(value: f64, precision: u8, max_idx: usize) -> (f64, usize) {
    let mut scaled = value;
    let mut idx = 0usize;

    while scaled >= 1_000.0 && idx < max_idx {
        scaled /= 1_000.0;
        idx += 1;
    }

    let scaled = round_to_non_negative(scaled, precision);

    if scaled >= 1_000.0 && idx < max_idx {
        (scaled / 1_000.0, idx + 1)
    } else {
        (scaled, idx)
    }
}

#[inline]
fn round_to_non_negative(value: f64, precision: u8) -> f64 {
    // IMPORTANT:
    // - This crate supports `no_std` on stable (MSRV 1.67).
    // - `core` does NOT provide `f64::round()`/`fract()` methods on stable.
    //
    // We therefore use a small, fast half-up rounding that only relies on:
    // - multiplication
    // - addition
    // - truncation via integer casts (safe for non-negative values)
    //
    // For extreme magnitudes (where the cast would saturate), we skip rounding.
    let p = (precision.min(6)) as usize;
    let factor = POW10_F64[p];

    if !value.is_finite() {
        return value;
    }

    // The float path only rounds non-negative values (we format abs() and emit the sign separately).
    if value < 0.0 {
        return value;
    }

    // Ensure the integer cast stays meaningful.
    // For very large values, rounding at `precision` digits is irrelevant anyway.
    let max_safe = (u128::MAX as f64) / factor;
    if value > max_safe {
        return value;
    }

    // Half-up rounding: for non-negative values, truncation == floor.
    // Example: 1.234 with precision=2 => (123.4 + 0.5) => 123.9 => 123 => 1.23
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
    // `input` is expected to be ASCII output from float formatting ("1234.5", "0", etc).
    // We keep this parser robust even if an exponent slips in for extreme values.
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
