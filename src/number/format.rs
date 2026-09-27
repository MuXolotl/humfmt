use core::fmt;

use crate::common::decimal::Digits;
use crate::common::fmt::{decimal_parts_rounded, write_frac_digits, write_scaled_integer};
use crate::common::numeric::NumericValue;
use crate::rounding::Precision;

use super::NumberOptions;

// Powers of 1000 as u128, for O(1) integer compact-unit selection.
// Index i corresponds to 1000^i.
//
// The table goes up to undecillion (10^36), which is enough to keep the full
// u128 range compact: u128::MAX is about 340.3 undecillion.
const POW1000: [u128; 13] = [
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
    1_000_000_000_000_000_000_000_000_000_000_000_000,
];

const SHORT_SUFFIXES: [&str; 13] = [
    "", "K", "M", "B", "T", "Qa", "Qi", "Sx", "Sp", "Oc", "No", "Dc", "Ud",
];

const LONG_SUFFIXES: [&str; 13] = [
    "",
    " thousand",
    " million",
    " billion",
    " trillion",
    " quadrillion",
    " quintillion",
    " sextillion",
    " septillion",
    " octillion",
    " nonillion",
    " decillion",
    " undecillion",
];

const MAX_SUFFIX_INDEX: usize = 12;

#[inline]
fn suffix_for(idx: usize, long: bool) -> &'static str {
    let table = if long {
        &LONG_SUFFIXES
    } else {
        &SHORT_SUFFIXES
    };

    if idx < table.len() {
        table[idx]
    } else {
        ""
    }
}

pub fn format_number<W: fmt::Write + ?Sized>(
    f: &mut W,
    value: NumericValue,
    options: &NumberOptions,
) -> fmt::Result {
    match value {
        NumericValue::Int(v) => format_int(f, v, options),
        NumericValue::UInt(v) => format_uint(f, v, options),
        NumericValue::Float(v) => format_float(f, v, options),
    }
}

fn format_int<W: fmt::Write + ?Sized>(
    f: &mut W,
    value: i128,
    options: &NumberOptions,
) -> fmt::Result {
    let negative = value.is_negative();
    let magnitude = value.unsigned_abs();

    format_u128_magnitude(f, negative && magnitude != 0, magnitude, options)
}

fn format_uint<W: fmt::Write + ?Sized>(
    f: &mut W,
    value: u128,
    options: &NumberOptions,
) -> fmt::Result {
    format_u128_magnitude(f, false, value, options)
}

fn format_u128_magnitude<W: fmt::Write + ?Sized>(
    f: &mut W,
    negative: bool,
    magnitude: u128,
    options: &NumberOptions,
) -> fmt::Result {
    let max_idx = if options.compact { MAX_SUFFIX_INDEX } else { 0 };

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

    // Rounding can push the integer part to the next threshold:
    // 999_950 at precision=1 becomes 1000K, which should render as 1M.
    if parts.integer.at_least(1_000) && idx < max_idx {
        idx += 1;
        unit = POW1000[idx];

        let res = get_parts(unit);
        decimals = res.0;
        parts = res.1;
    }

    if negative {
        f.write_char('-')?;
    } else if options.force_sign && magnitude != 0 {
        // Integer magnitudes >= 1 never round to zero, so magnitude != 0 is sufficient.
        f.write_char('+')?;
    }

    // Digit grouping applies only when the value is not compacted.
    write_scaled_integer(
        f,
        parts.integer,
        options.separators && idx == 0,
        options.group_separator,
    )?;

    write_int_frac(
        f,
        &parts,
        decimals,
        options.fixed_precision,
        options.decimal_separator,
    )?;

    f.write_str(suffix_for(idx, options.long_units))
}

// Selects the compact scale index for a u128 magnitude in O(1) via ilog10.
// Returns (index, divisor), where divisor = 1000^index.
#[inline]
fn compact_unit_for_u128(magnitude: u128, max_idx: usize) -> (usize, u128) {
    if magnitude < 1_000 || max_idx == 0 {
        return (0, 1);
    }

    let idx = ((magnitude.ilog10() / 3) as usize).min(max_idx);
    (idx, POW1000[idx])
}

fn format_float<W: fmt::Write + ?Sized>(
    f: &mut W,
    raw: f64,
    options: &NumberOptions,
) -> fmt::Result {
    if !raw.is_finite() {
        return write!(f, "{raw}");
    }

    let max_idx = if options.compact { MAX_SUFFIX_INDEX } else { 0 };

    let negative = raw.is_sign_negative();
    let (idx, digits, decimals) = scaled_digits(
        raw.abs(),
        &options.precision,
        max_idx,
        options.rounding,
        negative,
    );

    let is_zero = digits.is_zero();

    if negative && !is_zero {
        f.write_char('-')?;
    } else if options.force_sign && !negative && !is_zero {
        f.write_char('+')?;
    }

    digits.write(
        f,
        decimals as usize,
        options.fixed_precision,
        options.separators && idx == 0,
        options.group_separator,
        options.decimal_separator,
    )?;

    f.write_str(suffix_for(idx, options.long_units))
}

// Fraction digits a scaled value may show. The option setters clamp decimal
// precision to the same number.
const MAX_FRACTION_DIGITS: i32 = 6;

// Rounds `abs` into its display unit: the compact scale index, the rounded
// digits of the scaled value, and the number of fraction digits to show.
//
// The rounding runs on the exact decimal digits of the value, so a value that
// was written as a short decimal rounds the way it reads: `0.15` at one decimal
// place is `0.2`, not the `0.1` that the binary value really falls below.
fn scaled_digits(
    abs: f64,
    precision: &Precision,
    max_idx: usize,
    rounding: crate::RoundingMode,
    is_negative: bool,
) -> (usize, Digits, u8) {
    let hint = leading_place_hint(abs);
    let mut idx = compact_index(hint, max_idx);

    loop {
        let (digits, decimals) = round_scaled(abs, precision, idx, hint, rounding, is_negative);

        // Rounding the top of a unit can reach the start of the next one, which
        // is the scale those digits belong to.
        if idx < max_idx && digits.leading_place().is_some_and(|place| place >= 3) {
            idx += 1;
            continue;
        }

        return (idx, digits, decimals);
    }
}

// Rounds `abs` at the display place of scale index `idx`.
fn round_scaled(
    abs: f64,
    precision: &Precision,
    idx: usize,
    hint: i32,
    rounding: crate::RoundingMode,
    is_negative: bool,
) -> (Digits, u8) {
    let shift = 3 * idx as i32;
    let scaled_hint = hint - shift;

    // The digits are printed in the value's own scale and shifted afterwards.
    // Significant-digit rounding needs the leading digit, whose place decides
    // where the kept digits end, so the print has to reach it.
    let place_hint = match *precision {
        Precision::Decimals(places) => -(i32::from(places)),
        Precision::Significant(sig_figs) => {
            (scaled_hint - (i32::from(sig_figs) - 1)).max(-MAX_FRACTION_DIGITS)
        }
    };

    let mut digits = Digits::covering(abs, place_hint + shift, hint - 2);
    digits.shift(shift);

    let place = match (*precision, digits.leading_place()) {
        (Precision::Decimals(places), _) => -(i32::from(places)),
        (Precision::Significant(sig_figs), Some(leading)) => {
            (leading - (i32::from(sig_figs) - 1)).max(-MAX_FRACTION_DIGITS)
        }
        // Zero has no leading digit; keeping the request in the place makes
        // fixed precision pad zeros the same way for every value.
        (Precision::Significant(sig_figs), None) => -(i32::from(sig_figs) - 1),
    };

    digits.round_at(place, rounding, is_negative);

    (digits, (-place).max(0) as u8)
}

// Compact scale index that shows a value with the given leading decimal place.
fn compact_index(leading: i32, max_idx: usize) -> usize {
    if leading < 3 {
        return 0;
    }

    ((leading / 3) as usize).min(max_idx)
}

// Estimate of the leading decimal place of a positive finite value, within one
// place of the truth.
//
// The binary exponent gives it directly: `log10(2)` is `0.30103`, and the
// mantissa moves the result by less than one place.
fn leading_place_hint(abs: f64) -> i32 {
    let bits = abs.to_bits();
    let biased = ((bits >> 52) & 0x7FF) as i32;
    let fraction = bits & ((1u64 << 52) - 1);

    // A subnormal has no implicit bit, so its highest set bit is the exponent.
    let exponent = if biased == 0 {
        63 - fraction.leading_zeros() as i32 - 1074
    } else {
        biased - 1023
    };

    (exponent * 30103) / 100_000
}

// Writes the fractional part of a DecimalParts value.
fn write_int_frac<W: fmt::Write + ?Sized>(
    f: &mut W,
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
