//! Exactness of the floating-point formatting paths.
//!
//! The formatters round the exact value of their input, the way `core` rounds
//! one when it is asked for a fixed precision. Scaling with `f64` arithmetic
//! first would round twice and can move a value across a boundary that its
//! written form does not cross: `0.145 * 100.0` is exactly `14.5`, while the
//! value of `0.145` is `14.499999999999999…` per cent.
//!
//! Every expectation below is the exact decimal expansion of the input rounded
//! at the requested place, and the cross-check compares the number path against
//! `core`'s own fixed-precision rendering of that same value.

use humfmt::{number_with, percent_with, NumberOptions, PercentOptions, RoundingMode};

#[test]
fn rounds_the_exact_value_of_the_input() {
    // `0.15`, `1.15` and `0.35` are all just below their written form as
    // `f64`, so the dropped digit is a four and not a five.
    let one_decimal = NumberOptions::new().precision(1);
    assert_eq!(number_with(0.15_f64, one_decimal).to_string(), "0.1");
    assert_eq!(number_with(1.15_f64, one_decimal).to_string(), "1.1");
    assert_eq!(number_with(0.35_f64, one_decimal).to_string(), "0.3");

    // `2.675` is `2.67499999999999982236431605997495353221893310546875`.
    let two_decimals = NumberOptions::new().precision(2);
    assert_eq!(number_with(2.675_f64, two_decimals).to_string(), "2.67");
    assert_eq!(number_with(1.005_f64, two_decimals).to_string(), "1");

    // `1.0005` is `1.0004999999999999964472863211998737156391143798828125`,
    // and multiplying it by a thousand in `f64` lands exactly on a half.
    assert_eq!(
        number_with(1.0005_f64, NumberOptions::new().precision(3)).to_string(),
        "1"
    );

    // The largest `f64` below `0.5` raises the same half flag once `+ 0.5`
    // rounds up to `1.0`.
    assert_eq!(
        number_with(0.49999999999999994_f64, NumberOptions::new().precision(0)).to_string(),
        "0"
    );
}

#[test]
fn keeps_the_rounding_direction_of_the_exact_tail() {
    let floor = NumberOptions::new()
        .precision(2)
        .rounding(RoundingMode::Floor);
    let ceil = NumberOptions::new()
        .precision(2)
        .rounding(RoundingMode::Ceil);

    // `0.15` sits above `0.14` and below `0.15`, so it fills the lower place.
    assert_eq!(number_with(0.15_f64, floor).to_string(), "0.14");
    assert_eq!(number_with(0.15_f64, ceil).to_string(), "0.15");

    // The direction belongs to the signed value, so a negative magnitude still
    // floors towards negative infinity.
    assert_eq!(number_with(-0.15_f64, floor).to_string(), "-0.15");
    assert_eq!(number_with(-0.15_f64, ceil).to_string(), "-0.14");
}

#[test]
fn writes_values_past_the_f64_range_from_their_digits() {
    // `f64::MAX` rounded to one significant digit is `2 * 10^308`, larger than
    // any `f64`. The digits are written without computing that value: they are
    // the scaled digits of the value, so the largest unit leaves `10^272`
    // behind the leading two.
    let rounded = number_with(f64::MAX, NumberOptions::new().significant_digits(1)).to_string();
    let scaled = rounded
        .strip_suffix("Ud")
        .expect("the value uses the top unit");
    assert_eq!(scaled.len(), 273);
    assert!(scaled.starts_with('2'));
    assert!(scaled[1..].chars().all(|digit| digit == '0'));

    // The percentage path moves the same digits two places further.
    let percented = percent_with(f64::MAX, PercentOptions::new().precision(0)).to_string();
    assert!(percented.starts_with("17976931348623157081452742373170435679807056752584499659891"));
    assert!(percented.ends_with('%'));
    assert_eq!(percented.len(), 311 + "%".len());
}

#[test]
fn carries_into_the_first_integer_place_without_a_leading_zero() {
    // A ratio far below one percent ceils to a single percent. Its leading
    // digit sat behind the decimal point before the scale-up, and the zero in
    // front of it must not reach the output.
    let ceil = PercentOptions::new()
        .precision(0)
        .rounding(RoundingMode::Ceil);
    assert_eq!(percent_with(1e-300_f64, ceil).to_string(), "1%");

    let floor = PercentOptions::new()
        .precision(0)
        .rounding(RoundingMode::Floor);
    assert_eq!(percent_with(-1e-300_f64, floor).to_string(), "-1%");

    let fine = PercentOptions::new()
        .precision(6)
        .rounding(RoundingMode::Ceil);
    assert_eq!(percent_with(1e-300_f64, fine).to_string(), "0.000001%");

    // The number path rounds the same value up at the sixth decimal place.
    let number_ceil = NumberOptions::new()
        .precision(6)
        .rounding(RoundingMode::Ceil);
    assert_eq!(number_with(1e-300_f64, number_ceil).to_string(), "0.000001");
}

#[test]
fn matches_core_fixed_precision_rounding() {
    for &value in AWKWARD.iter() {
        for decimals in 0..=6 {
            let ours = number_with(
                value,
                NumberOptions::new()
                    .precision(decimals as u8)
                    .compact(false),
            )
            .to_string();
            let core = format!("{value:.decimals$}");
            let core = trimmed(&core);

            if ours == core {
                continue;
            }

            // A value that rounds to zero keeps no sign, while `core` renders
            // negative zero as `-0`.
            if ours == "0" && core == "-0" {
                continue;
            }

            // `core` keeps the even digit on a tie and `HalfUp` moves away from
            // zero, so a difference is only allowed on an exact tie and only by
            // the single last place.
            assert!(
                is_exact_tie(value, decimals),
                "{value} at {decimals} decimals: {ours} != {core}"
            );
            assert_eq!(
                scaled(&ours, decimals),
                scaled(core, decimals) + 1,
                "{value} at {decimals} decimals: {ours} != {core}"
            );
        }
    }
}

/// Values whose exact binary value sits just off a short decimal, the integer
/// values that lose every fraction, and the ends of the exponent range.
const AWKWARD: [f64; 27] = [
    0.0,
    0.1,
    0.15,
    0.2,
    0.25,
    0.35,
    0.4,
    0.45,
    0.5,
    0.05,
    0.005,
    1.005,
    1.0005,
    1.15,
    2.5,
    2.675,
    0.425,
    0.4235,
    0.4255,
    0.99999,
    999.95,
    12_345.678,
    123_456.789,
    -2.675,
    -0.15,
    // 2^53 - 1: the last integer whose neighbours are one apart.
    9_007_199_254_740_991.0,
    1e22,
];

/// Drops the trailing fraction zeros and a bare decimal point, the way the
/// formatter does when `fixed_precision` is off.
fn trimmed(text: &str) -> &str {
    let Some((integer, fraction)) = text.split_once('.') else {
        return text;
    };

    let fraction = fraction.trim_end_matches('0');

    if fraction.is_empty() {
        integer
    } else {
        &text[..integer.len() + 1 + fraction.len()]
    }
}

/// Reads a rendered value as the integer it is worth in units of `10^-decimals`
/// without going through `f64`.
fn scaled(text: &str, decimals: usize) -> u128 {
    let text = text.trim_start_matches('-');
    let (integer, fraction) = text.split_once('.').unwrap_or((text, ""));

    let mut digits = integer.to_owned();

    digits.push_str(fraction);
    digits.extend(core::iter::repeat('0').take(decimals - fraction.len()));

    digits.parse().expect("the value fits in u128")
}

/// Returns `true` when the exact expansion of `value` ends in a five at
/// `decimals + 1` places, which is what makes the rounding a tie.
///
/// Removing the factors of two from the mantissa gives the exponent of the odd
/// part, and the expansion of `odd * 2^e` ends at place `e`.
fn is_exact_tie(value: f64, decimals: usize) -> bool {
    let bits = value.abs().to_bits();
    let biased = ((bits >> 52) & 0x7FF) as i32;
    let fraction = bits & ((1u64 << 52) - 1);

    let (mantissa, exponent) = if biased == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1u64 << 52), biased - 1075)
    };

    exponent + mantissa.trailing_zeros() as i32 == -(decimals as i32) - 1
}
