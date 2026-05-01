use core::fmt;
use core::fmt::Write;

use crate::common::fmt::write_u128;
use crate::locale::Locale;

use super::PercentOptions;

// Powers of 10 used for rounding, indexed by precision (0..=6).
const POW10: [u64; 7] = [1, 10, 100, 1_000, 10_000, 100_000, 1_000_000];

pub fn format_percent<L: Locale>(
    f: &mut fmt::Formatter<'_>,
    value: f64,
    options: &PercentOptions<L>,
) -> fmt::Result {
    // Non-finite values render as-is with a percent sign.
    if !value.is_finite() {
        return write!(f, "{value}%");
    }

    let percent = value * 100.0;
    let negative = percent.is_sign_negative();
    let abs = percent.abs();
    let precision = options.precision;
    let decimal_sep = options.locale.decimal_separator();

    // Round to the requested precision using half-up integer arithmetic.
    // Mirrors round_f64 in number/format.rs but stays local to avoid
    // coupling the two modules.
    let factor = POW10[precision as usize] as f64;
    let rounded = if abs * factor + 0.5 < u64::MAX as f64 {
        let shifted = (abs * factor + 0.5) as u64;
        shifted as f64 / factor
    } else {
        abs
    };

    // Suppress negative zero.
    let is_zero = rounded == 0.0;
    if negative && !is_zero {
        f.write_char('-')?;
    }

    // Split into integer and fractional parts.
    let int_part = rounded as u128;
    let frac_f = rounded - int_part as f64;

    write_u128(f, int_part, false, options.locale.group_separator())?;

    if precision > 0 {
        let p = precision as usize;
        let frac_shifted = (frac_f * factor + 0.5) as u64;
        // Clamp to avoid carry overflow (e.g. 0.9999... * 10 + 0.5 rounds up to 10).
        let frac_clamped = frac_shifted.min(factor as u64 - 1);

        let mut buf = [b'0'; 6];
        let mut rem = frac_clamped;
        for i in (0..p).rev() {
            buf[i] = b'0' + (rem % 10) as u8;
            rem /= 10;
        }

        if options.fixed_precision {
            f.write_char(decimal_sep)?;
            let s = unsafe { core::str::from_utf8_unchecked(&buf[..p]) };
            f.write_str(s)?;
        } else {
            // Trim trailing zeros.
            let mut end = p;
            while end > 0 && buf[end - 1] == b'0' {
                end -= 1;
            }
            if end > 0 {
                f.write_char(decimal_sep)?;
                let s = unsafe { core::str::from_utf8_unchecked(&buf[..end]) };
                f.write_str(s)?;
            }
        }
    }

    f.write_char('%')
}
