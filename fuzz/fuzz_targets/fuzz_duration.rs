#![no_main]

use core::time::Duration;
use humfmt::{duration_with, DurationOptions};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 9 {
        return;
    }

    let nanos = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let max_units = ((data[8] & 0b0000_0111) + 1).min(7);
    let long_units = (data[8] & 0b0000_1000) != 0;

    let mut opts = DurationOptions::new().max_units(max_units);

    if long_units {
        opts = opts.long_units();
    }

    let duration = Duration::from_nanos(nanos);
    let _ = duration_with(duration, opts).to_string();
});
