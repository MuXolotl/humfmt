#![no_main]

use humfmt::{bytes_with, ByteUnit, BytesOptions, RoundingMode};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 12 {
        return;
    }

    let value = i64::from_le_bytes(data[0..8].try_into().unwrap());
    let flags = data[8];
    let unit_selector = data[9];

    let precision = flags & 0b0000_0111;
    let binary = (flags & 0b0000_1000) != 0;
    let bits_mode = (flags & 0b0001_0000) != 0;
    let space = (flags & 0b0010_0000) != 0;
    let long_units = (flags & 0b0100_0000) != 0;
    let fixed = (flags & 0b1000_0000) != 0;

    let rounding = match data[10] & 0b11 {
        0 => RoundingMode::HalfUp,
        1 => RoundingMode::Floor,
        _ => RoundingMode::Ceil,
    };

    let min_unit = match unit_selector % 7 {
        0 => ByteUnit::B,
        1 => ByteUnit::KB,
        2 => ByteUnit::MB,
        3 => ByteUnit::GB,
        4 => ByteUnit::TB,
        5 => ByteUnit::PB,
        _ => ByteUnit::EB,
    };

    let max_unit = match (unit_selector / 7) % 7 {
        0 => ByteUnit::B,
        1 => ByteUnit::KB,
        2 => ByteUnit::MB,
        3 => ByteUnit::GB,
        4 => ByteUnit::TB,
        5 => ByteUnit::PB,
        _ => ByteUnit::EB,
    };

    let mut opts = BytesOptions::new()
        .precision(precision.min(6))
        .bits(bits_mode)
        .space(space)
        .fixed_precision(fixed)
        .rounding(rounding)
        .min_unit(min_unit)
        .max_unit(max_unit);

    if binary {
        opts = opts.binary();
    }
    if long_units {
        opts = opts.long_units();
    }

    let _ = bytes_with(value, opts).to_string();
});
