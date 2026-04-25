use core::fmt;

use crate::common::numeric::NumericValue;

use super::NumberOptions;

pub fn format_number<L: crate::locale::Locale>(
    f: &mut fmt::Formatter<'_>,
    value: NumericValue,
    options: &NumberOptions<L>,
) -> fmt::Result {
    let raw = to_f64(value);

    if !raw.is_finite() {
        return write!(f, "{raw}");
    }

    let negative = raw.is_sign_negative() && raw != 0.0;
    let abs = raw.abs();

    let (scaled, idx) = normalize_scaled(
        abs,
        options.precision_value(),
        options.locale_ref().max_compact_suffix_index(),
    );

    let rendered = render_scaled(
        scaled,
        options.precision_value(),
        options.separators_value(),
    );

    if negative {
        write!(f, "-")?;
    }

    write!(f, "{rendered}")?;

    let suffix = options
        .locale_ref()
        .compact_suffix(idx, options.long_units_value());

    write!(f, "{suffix}")
}

fn to_f64(value: NumericValue) -> f64 {
    match value {
        NumericValue::Int(v) => v as f64,
        NumericValue::UInt(v) => v as f64,
        NumericValue::Float(v) => v,
    }
}

fn normalize_scaled(value: f64, precision: u8, max_idx: usize) -> (f64, usize) {
    let mut scaled = value;
    let mut idx = 0;

    while scaled >= 1_000.0 && idx < max_idx {
        scaled /= 1_000.0;
        idx += 1;
    }

    scaled = round_to(scaled, precision);

    if scaled >= 1_000.0 && idx < max_idx {
        scaled /= 1_000.0;
        idx += 1;
    }

    (scaled, idx)
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

fn render_scaled(value: f64, precision: u8, separators: bool) -> alloc::string::String {
    let mut out = if is_integer(value) {
        alloc::format!("{:.0}", value)
    } else {
        alloc::format!("{:.*}", precision as usize, value)
    };

    trim_trailing_zeroes(&mut out);

    if separators {
        out = add_separators(&out);
    }

    out
}

fn is_integer(value: f64) -> bool {
    value == (value as u128) as f64
}

fn trim_trailing_zeroes(s: &mut alloc::string::String) {
    if !s.contains('.') {
        return;
    }

    while s.ends_with('0') {
        s.pop();
    }

    if s.ends_with('.') {
        s.pop();
    }
}

fn add_separators(input: &str) -> alloc::string::String {
    let mut split = input.split('.');
    let int_part = split.next().unwrap_or("");
    let frac_part = split.next();

    let mut out = alloc::string::String::new();
    let chars: alloc::vec::Vec<char> = int_part.chars().rev().collect();

    for (i, ch) in chars.iter().enumerate() {
        if i != 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(*ch);
    }

    let mut int_done: alloc::string::String = out.chars().rev().collect();

    if let Some(frac) = frac_part {
        int_done.push('.');
        int_done.push_str(frac);
    }

    int_done
}
