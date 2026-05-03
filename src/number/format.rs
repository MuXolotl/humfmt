use core::fmt;
use core::fmt::Write;

use crate::common::fmt::{
    decimal_parts_rounded, write_frac_digits, write_grouped_ascii_digits, write_u128, StackString,
};
use crate::common::numeric::NumericValue;

use super::options::Precision;
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
    let max_idx = if options.compact {
        locale.max_compact_suffix_index().min(POW1000.len() - 1)
    } else {
        0
    };

    let (mut idx, mut unit) = compact_unit_for_u128(magnitude, max_idx);

    let get_parts = |u: u128| match options.precision {
        Precision::Decimals(p) => (
            p,
            decimal_parts_rounded(magnitude, u, p, options.rounding, negative),
        ),
        Precision::Significant(n) => {
            crate::common::fmt::compute_sigfigs_u128(magnitude, u, n, options.rounding, negative)
        }
    };

    let (mut decimals, mut parts) = get_parts(unit);

    // Rounding can push the integer part to the next threshold (e.g. 999_950
    // at precision=1 rounds to 1000K → rescale to 1M). Adjust once.
    if parts.integer >= 1_000 && idx < max_idx {
        idx += 1;
        unit = POW1000[idx];
        let res = get_parts(unit);
        decimals = res.0;
        parts = res.1;
    }

    if negative {
        f.write_char('-')?;
    }

    // Digit grouping separators only apply when the value is unscaled (idx == 0).
    write_u128(
        f,
        parts.integer,
        options.separators && idx == 0,
        locale.group_separator(),
    )?;

    write_int_frac(
        f,
        &parts,
        decimals,
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
        return write!(f, "{raw}");
    }

    let locale = &options.locale;
    let max_idx = if options.compact {
        locale.max_compact_suffix_index().min(POW1000_F64.len() - 1)
    } else {
        0
    };

    let negative = raw.is_sign_negative();
    let abs = raw.abs();

    let (idx, decimals, scaled_abs) =
        compact_unit_for_f64(abs, &options.precision, max_idx, options.rounding, negative);

    // Suppress negative zero
    let is_zero = scaled_abs == 0.0;
    if negative && !is_zero {
        f.write_char('-')?;
    }

    // Stack buffer expanded to 512 bytes to safely support compact(false)
    // with very large non-exponential f64 values (up to 10^308).
    let mut buf = StackString::<512>::new();
    write!(&mut buf, "{:.*}", decimals as usize, scaled_abs)
        .expect("StackString<512> overflow is impossible for valid display f64");

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
fn compact_unit_for_f64(
    abs: f64,
    precision: &Precision,
    max_idx: usize,
    rounding: crate::RoundingMode,
    is_negative: bool,
) -> (usize, u8, f64) {
    let get_scaled = |abs_val: f64| match *precision {
        Precision::Decimals(d) => (d, round_f64(abs_val, d, rounding, is_negative)),
        Precision::Significant(s) => compute_sigfigs_f64(abs_val, s, rounding, is_negative),
    };

    if abs < 1_000.0 || max_idx == 0 {
        let (decimals, scaled) = get_scaled(abs);
        // Rounding can push a value just below 1000 to exactly 1000.
        if scaled >= 1_000.0 && max_idx > 0 {
            let (d2, s2) = get_scaled(abs / 1_000.0);
            return (1, d2, s2);
        }
        return (0, decimals, scaled);
    }

    let bits = abs.to_bits();
    let biased_exp = ((bits >> 52) & 0x7FF) as i32;
    let unbiased_exp = biased_exp - 1023;

    let approx_idx = if unbiased_exp <= 0 {
        0usize
    } else {
        ((unbiased_exp as usize) * 1_000 / 9_966).min(max_idx)
    };

    let mut idx =
        if approx_idx > 0 && approx_idx < POW1000_F64.len() && abs < POW1000_F64[approx_idx] {
            approx_idx - 1
        } else {
            approx_idx
        }
        .min(max_idx);

    let divisor = POW1000_F64[idx.min(POW1000_F64.len() - 1)];
    let (mut decimals, mut scaled) = get_scaled(abs / divisor);

    if scaled >= 1_000.0 && idx < max_idx {
        idx += 1;
        let res = get_scaled(abs / POW1000_F64[idx]);
        decimals = res.0;
        scaled = res.1;
    }

    (idx, decimals, scaled)
}

fn compute_sigfigs_f64(
    abs: f64,
    sig_figs: u8,
    rounding: crate::RoundingMode,
    negative: bool,
) -> (u8, f64) {
    if abs == 0.0 {
        return (sig_figs.saturating_sub(1), 0.0);
    }

    let log10 = f64_log10_floor(abs);
    let shift = sig_figs as i32 - 1 - log10;

    if shift >= 0 {
        let decimals = (shift as u8).min(6);
        let rounded = round_f64(abs, decimals, rounding, negative);

        let new_log10 = if rounded > 0.0 {
            f64_log10_floor(rounded)
        } else {
            log10
        };

        if new_log10 > log10 {
            let new_shift = sig_figs as i32 - 1 - new_log10;
            let new_decimals = if new_shift >= 0 {
                (new_shift as u8).min(6)
            } else {
                0
            };
            return (new_decimals, rounded);
        }
        (decimals, rounded)
    } else {
        let drop_digits = -shift;
        let factor = f64_pow10(drop_digits);
        let divided = abs / factor;
        let rounded = round_f64(divided, 0, rounding, negative);
        (0, rounded * factor)
    }
}

// A no_std compatible base-10 exponentiation implementation.
#[inline]
fn f64_pow10(mut exp: i32) -> f64 {
    let mut res = 1.0;
    let is_neg = exp < 0;
    exp = exp.abs();

    let mut base = 10.0;
    while exp > 0 {
        if exp % 2 == 1 {
            res *= base;
        }
        base *= base;
        exp /= 2;
    }

    if is_neg {
        1.0 / res
    } else {
        res
    }
}

// A no_std compatible base-10 logarithmic approximation based on IEEE 754 exponents.
#[inline]
fn f64_log10_floor(val: f64) -> i32 {
    if val <= 0.0 {
        return 0;
    }

    let bits = val.to_bits();
    let exp = ((bits >> 52) & 0x7FF) as i32 - 1023;

    // Use standard math constant for log10(2) to satisfy clippy
    let log2_val = exp as f64 * core::f64::consts::LOG10_2;
    let mut approx = log2_val as i32;

    // Adjust for truncation of negative numbers.
    if log2_val < 0.0 && log2_val != approx as f64 {
        approx -= 1;
    }

    let p = f64_pow10(approx);
    let p_next = f64_pow10(approx + 1);

    if val < p {
        approx -= 1;
    } else if val >= p_next {
        approx += 1;
    }

    approx
}

// Rounds a non-negative finite f64 to `precision` decimal places based on the `RoundingMode`.
//
// Uses integer-cast rounding instead of f64::round() because round() is not
// available in core on stable no_std at MSRV 1.70.
#[inline]
fn round_f64(value: f64, precision: u8, rounding: crate::RoundingMode, is_negative: bool) -> f64 {
    debug_assert!(value.is_finite() && value >= 0.0);

    let p = precision.min(6) as usize;
    let factor = POW10_F64[p];

    if value > f64::MAX / factor {
        return value;
    }

    let shifted = value * factor;
    let trunc = shifted as u64;

    if trunc as f64 >= u64::MAX as f64 {
        return value;
    }

    let has_remainder = shifted > trunc as f64;

    let carry = match rounding {
        crate::RoundingMode::HalfUp => {
            let half_shifted = shifted + 0.5;
            (half_shifted as u64) > trunc
        }
        crate::RoundingMode::Floor => is_negative && has_remainder,
        crate::RoundingMode::Ceil => !is_negative && has_remainder,
    };

    let rounded_int = if carry { trunc + 1 } else { trunc };
    rounded_int as f64 / factor
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
