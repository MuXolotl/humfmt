#![no_main]

use humfmt::{list_with, ListOptions};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    let items: Vec<String> = data
        .chunks(3)
        .take(6)
        .map(|chunk| {
            let val = u32::from_le_bytes([
                chunk[0],
                chunk.get(1).copied().unwrap_or(0),
                chunk.get(2).copied().unwrap_or(0),
                0,
            ]);
            format!("item_{val}")
        })
        .collect();

    if items.is_empty() {
        return;
    }

    let flags = data[0];
    let serial_comma = (flags & 0b0000_0001) != 0;
    let custom_conjunction = (flags & 0b0000_0010) != 0;

    let mut opts = ListOptions::new().serial_comma_enabled(serial_comma);

    if custom_conjunction {
        opts = opts.conjunction("plus");
    }

    let _ = list_with(&items, opts).to_string();
});
