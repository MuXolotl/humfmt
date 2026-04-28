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
    // Note: `ilog10` is stable and fast, but is undefined for zero. We already
    // handled magnitude < 1000, which includes zero.
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
    // Buffer size is intentionally generous to avoid fmt::Error for extreme floats.
    let mut tmp = StackString::<512>::new();
    if is_integer(scaled) {
        write!(&mut tmp, "{scaled:.0}")?;
    } else {
        write!(&mut tmp, "{:.*}", precision as usize, scaled)?;
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

    let scaled = round_to(scaled, precision);

    if scaled >= 1_000.0 && idx < max_idx {
        (scaled / 1_000.0, idx + 1)
    } else {
        (scaled, idx)
    }
}

fn round_to(value: f64, precision: u8) -> f64 {
    let factor = pow10(precision);
    (((value * factor) + 0.5) as u128 as f64) / factor
}

fn pow10(precision: u8) -> f64 {
    let mut factor = 1.0;
    for _ in 0..precision {
        factor *= 10.0;
    }
    factor
}

fn is_integer(value: f64) -> bool {
    value == (value as u128) as f64
}

fn write_localized_numeric_str(
    f: &mut fmt::Formatter<'_>,
    input: &str,
    separators: bool,
    decimal_separator: char,
    group_separator: char,
) -> fmt::Result {
    // `input` is expected to be ASCII output from float formatting ("1234.5", "0", etc).
    let (int_part, frac_part) = match input.split_once('.') {
        Some((a, b)) => (a, Some(b)),
        None => (input, None),
    };

    if separators {
        crate::common::fmt::write_grouped_ascii_digits(f, int_part, group_separator)?;
    } else {
        f.write_str(int_part)?;
    }

    if let Some(frac) = frac_part {
        f.write_char(decimal_separator)?;
        f.write_str(frac)?;
    }

    Ok(())
}
