use core::fmt;

use crate::common::decimal::Digits;

use super::PercentOptions;

pub fn format_percent<W: fmt::Write + ?Sized>(
    f: &mut W,
    value: f64,
    options: &PercentOptions,
) -> fmt::Result {
    if !value.is_finite() {
        return write!(f, "{value}%");
    }

    let negative = value.is_sign_negative();
    let precision = i32::from(options.precision);

    // A percentage is the ratio scaled by a hundred. The scaling moves the
    // decimal point of the exact digits instead of multiplying the binary
    // value, which keeps it exact: `0.29 * 100.0` is `28.999999999999996`.
    let mut digits = Digits::new(value.abs(), -(precision + 2));
    digits.shift(-2);
    digits.round_at(-precision, options.rounding, negative);

    let is_zero = digits.is_zero();

    if negative && !is_zero {
        f.write_char('-')?;
    } else if options.force_sign && !negative && !is_zero {
        f.write_char('+')?;
    }

    digits.write(
        f,
        precision as usize,
        options.fixed_precision,
        false,
        ',',
        options.decimal_separator,
    )?;

    f.write_char('%')
}
