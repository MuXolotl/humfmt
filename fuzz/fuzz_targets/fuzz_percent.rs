#![no_main]

use humfmt::{percent_with, PercentOptions};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 10 {
        return;
    }

    let value = f64::from_le_bytes(data[0..8].try_into().unwrap());
    let flags = data[8];

    let precision = flags & 0b0000_0111;
    let force_sign = (flags & 0b0000_1000) != 0;
    let fixed_precision = (flags & 0b0001_0000) != 0;

    let opts = PercentOptions::new()
        .precision(precision.min(6))
        .force_sign(force_sign)
        .fixed_precision(fixed_precision);

    let _ = percent_with(value, opts).to_string();
});
