use core::fmt;
use core::fmt::Write;

use crate::common::fmt::{
    decimal_parts_rounded, write_frac_digits, write_grouped_ascii_digits, write_u128, StackString,
};
use crate::common::numeric::NumericValue;

use super::NumberOptions;

// Powers of 1000 as u128, for O(1) integer compact-unit selection.
// Index i corresponds to 1000^i. The array covers up to decillion (10^33).
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

// Powers of 1000 as f64, for O(1) float compact-unit selection.
// Stored separately because f64 cannot exactly represent large u128 values.
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

// Powers of 10 as f64, indexed by precision (0..=6).
// Used for rounding floats to a fixed number of decimal places.
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
    let magnitude = value.unsigned_abs();
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

    let (mut idx, unit) = compact_unit_for_u128(magnitude, max_idx);
    let mut parts = decimal_parts_rounded(magnitude, unit, precision);

    // Rounding can push the integer part to the next threshold (e.g. 999_950
    // at precision=1 rounds to 1000K → rescale to 1M). Adjust once.
    if parts.integer >= 1_000 && idx < max_idx {
        idx += 1;
        parts = decimal_parts_rounded(magnitude, POW1000[idx], precision);
    }

    if negative {
        f.write_char('-')?;
    }

    // Digit grouping separators only apply when the value is unscaled (idx == 0).
    // For compacted output like "15.3K", grouping the integer part would produce
    // surprising results and is not useful.
    write_u128(
        f,
        parts.integer,
        options.separators && idx == 0,
        locale.group_separator(),
    )?;

    write_int_frac(
        f,
        &parts,
        precision,
        options.fixed_precision,
        locale.decimal_separator(),
    )?;

    let suffix = locale.compact_suffix_for(idx, parts.as_f64(), options.long_units);
    f.write_str(suffix)
}

// Selects the compact scale index for a u128 magnitude in O(1) via ilog10.
// Returns (index, divisor) where divisor = 1000^index.
#[inline]
fn compact_unit_for_u128(magnitude: u128, max_idx: usize) -> (usize, u128) {
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
        // Non-finite values use Rust's default float Display (locale-agnostic).
        // This produces "inf", "-inf", "NaN".
        return write!(f, "{raw}");
    }

    let locale = &options.locale;
    let precision = options.precision;
    let max_idx = locale.max_compact_suffix_index().min(POW1000_F64.len() - 1);

    let negative = raw.is_sign_negative();
    let abs = raw.abs();

    let (idx, scaled_abs) = compact_unit_for_f64(abs, precision, max_idx);

    // Suppress negative zero: a small negative value that rounds to 0 should
    // display as "0", not "-0".
    let is_zero = scaled_abs == 0.0;
    if negative && !is_zero {
        f.write_char('-')?;
    }

    // Write the scaled float into a stack buffer for trimming and locale
    // separator substitution. 64 bytes is always sufficient: precision is
    // capped at 6, so the longest possible output is "999.123456" (10 chars).
    let mut buf = StackString::<64>::new();
    write!(&mut buf, "{:.*}", precision as usize, scaled_abs)
        .expect("StackString<64> overflow is impossible for precision <= 6");

    write_localized_float_str(
        f,
        buf.as_str(),
        options.separators && idx == 0,
        options.fixed_precision,
        locale.decimal_separator(),
        locale.group_separator(),
    )?;

    let suffix = locale.compact_suffix_for(idx, scaled_abs, options.long_units);
    f.write_str(suffix)
}

// Selects the compact scale index and the rounded scaled value for a
// non-negative finite f64. Uses the IEEE 754 binary exponent for O(1) index
// estimation, then corrects by at most one step for floating-point imprecision.
//
// The binary exponent of a normal f64 is stored in bits 52..=62 with bias 1023.
// Dividing the unbiased exponent by log2(1000) ≈ 9.966 gives the approximate
// base-1000 index. Integer arithmetic (×1000/9966) avoids f64 here.
fn compact_unit_for_f64(abs: f64, precision: u8, max_idx: usize) -> (usize, f64) {
    if abs < 1_000.0 || max_idx == 0 {
        let scaled = round_f64(abs, precision);
        // Rounding can push a value just below 1000 to exactly 1000.
        return if scaled >= 1_000.0 && max_idx > 0 {
            (1, scaled / 1_000.0)
        } else {
            (0, scaled)
        };
    }

    let bits = abs.to_bits();
    let biased_exp = ((bits >> 52) & 0x7FF) as i32;
    let unbiased_exp = biased_exp - 1023;

    let approx_idx = if unbiased_exp <= 0 {
        0usize
    } else {
        ((unbiased_exp as usize) * 1_000 / 9_966).min(max_idx)
    };

    // Correct downward by one if the approximation overshot.
    let idx =
        if approx_idx > 0 && approx_idx < POW1000_F64.len() && abs < POW1000_F64[approx_idx] {
            approx_idx - 1
        } else {
            approx_idx
        }
        .min(max_idx);

    let divisor = POW1000_F64[idx.min(POW1000_F64.len() - 1)];
    let scaled = round_f64(abs / divisor, precision);

    // Rounding can push scaled to exactly 1000 — rescale once.
    if scaled >= 1_000.0 && idx < max_idx {
        (idx + 1, scaled / 1_000.0)
    } else {
        (idx, scaled)
    }
}

// Rounds a non-negative finite f64 to `precision` decimal places (half-up).
//
// Uses integer-cast rounding instead of f64::round() because round() is not
// available in core on stable no_std at MSRV 1.70.
// The cast `(x + 0.5) as u64` is correct for non-negative finite values in
// the safe range (scaled compact numbers are always well within u64::MAX).
#[inline]
fn round_f64(value: f64, precision: u8) -> f64 {
    debug_assert!(value.is_finite() && value >= 0.0);

    let p = precision.min(6) as usize;
    let factor = POW10_F64[p];

    if value > f64::MAX / factor {
        return value;
    }

    let shifted = value * factor + 0.5;
    if shifted >= u64::MAX as f64 {
        return value;
    }

    (shifted as u64) as f64 / factor
}

// Writes the fractional part of a DecimalParts value (integer path).
// Handles both trimmed (default) and fixed_precision modes.
fn write_int_frac(
    f: &mut fmt::Formatter<'_>,
    parts: &crate::common::fmt::DecimalParts,
    precision: u8,
    fixed_precision: bool,
    decimal_separator: char,
) -> fmt::Result {
    if fixed_precision {
        if precision > 0 {
            f.write_char(decimal_separator)?;
            let existing = parts.frac_len as usize;
            write_frac_digits(f, &parts.frac_digits[..existing])?;
            for _ in existing..precision as usize {
                f.write_char('0')?;
            }
        }
    } else if parts.frac_len != 0 {
        f.write_char(decimal_separator)?;
        write_frac_digits(f, &parts.frac_digits[..parts.frac_len as usize])?;
    }
    Ok(())
}

// Writes a float string (produced by Rust's "{:.*}" formatter) with:
// - locale-aware decimal separator substitution
// - optional digit grouping on the integer part
// - trailing-zero trimming (unless fixed_precision is set)
//
// Rust's float Display for values in 0..1000 never produces exponent notation,
// so we do not handle 'e'/'E' here.
fn write_localized_float_str(
    f: &mut fmt::Formatter<'_>,
    input: &str,
    group: bool,
    fixed_precision: bool,
    decimal_separator: char,
    group_separator: char,
) -> fmt::Result {
    let (int_part, frac_part) = match input.split_once('.') {
        Some((a, b)) => (a, Some(b)),
        None => (input, None),
    };

    if group {
        write_grouped_ascii_digits(f, int_part, group_separator)?;
    } else {
        f.write_str(int_part)?;
    }

    if let Some(frac) = frac_part {
        let trimmed = if fixed_precision {
            frac
        } else {
            frac.trim_end_matches('0')
        };
        if !trimmed.is_empty() {
            f.write_char(decimal_separator)?;
            f.write_str(trimmed)?;
        }
    }

    Ok(())
}
