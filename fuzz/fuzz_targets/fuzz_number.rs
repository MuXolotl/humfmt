#![no_main]

use humfmt::{number_with, NumberOptions, RoundingMode};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 10 {
        return;
    }

    let raw_value = i64::from_le_bytes(data[0..8].try_into().unwrap());
    let flags = data[8];
    let extra = if data.len() > 9 { data[9] } else { 0 };

    let precision = flags & 0b0000_0111;
    let sig_digits = (flags >> 3) & 0b0000_0111;
    let rounding = match (flags >> 6) & 0b11 {
        0 => RoundingMode::HalfUp,
        1 => RoundingMode::Floor,
        _ => RoundingMode::Ceil,
    };

    let compact = (extra & 0b0000_0001) != 0;
    let force_sign = (extra & 0b0000_0010) != 0;
    let separators = (extra & 0b0000_0100) != 0;
    let long_units = (extra & 0b0000_1000) != 0;
    let fixed_precision = (extra & 0b0001_0000) != 0;

    let mut opts = NumberOptions::new()
        .precision(precision.min(6))
        .rounding(rounding)
        .compact(compact)
        .force_sign(force_sign)
        .separators(separators)
        .fixed_precision(fixed_precision);

    if long_units {
        opts = opts.long_units();
    }

    if sig_digits > 0 {
        opts = opts.significant_digits(sig_digits.clamp(1, 39));
    }

    let _ = number_with(raw_value, opts).to_string();

    if data.len() >= 26 {
        let big = u128::from_le_bytes(data[10..26].try_into().unwrap());
        let _ = number_with(big, NumberOptions::new()).to_string();
    }

    if data.len() >= 18 {
        let float_val = f64::from_le_bytes(data[10..18].try_into().unwrap());
        let _ = number_with(float_val, opts).to_string();
    }
});
