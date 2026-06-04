#![no_main]

use humfmt::ordinal;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() < 8 {
        return;
    }

    let value = i64::from_le_bytes(data[0..8].try_into().unwrap());
    let _ = ordinal(value).to_string();

    if data.len() > 8 {
        let unsigned = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let _ = humfmt::ordinal::ordinal_suffix(unsigned as u128);
    }
});
