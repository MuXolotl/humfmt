use core::time::Duration;

use humfmt::{ago_with, bytes, duration_with, list_with, number, number_with};
use humfmt::{DurationOptions, ListOptions, NumberOptions};
use proptest::prelude::*;

const EN_SHORT_SUFFIXES: [&str; 12] = [
    "", "K", "M", "B", "T", "Qa", "Qi", "Sx", "Sp", "Oc", "No", "Dc",
];
const BYTE_SUFFIXES: [&str; 7] = ["B", "KB", "MB", "GB", "TB", "PB", "EB"];

fn count_rendered_units(rendered: &str) -> usize {
    rendered
        .split_whitespace()
        .filter(|token| {
            token
                .chars()
                .next()
                .map(|ch| ch.is_ascii_digit())
                .unwrap_or(false)
        })
        .count()
}

fn number_suffix_index(rendered: &str) -> usize {
    EN_SHORT_SUFFIXES
        .iter()
        .enumerate()
        .rfind(|(_, suffix)| rendered.ends_with(*suffix))
        .map(|(idx, _)| idx)
        .unwrap_or(0)
}

fn byte_suffix_index(rendered: &str) -> usize {
    BYTE_SUFFIXES
        .iter()
        .enumerate()
        .rfind(|(_, suffix)| rendered.ends_with(*suffix))
        .map(|(idx, _)| idx)
        .unwrap_or(0)
}

proptest! {
    #[test]
    fn number_sign_symmetry_holds_for_nonzero_integers(value in 1i64..=i64::MAX) {
        let positive = number(value).to_string();
        let negative = number(-value).to_string();

        prop_assert_eq!(negative, format!("-{positive}"));
    }

    #[test]
    fn tiny_negative_numbers_do_not_render_negative_zero(
        precision in 0u8..=6,
        step in 1u32..=9_999,
    ) {
        let factor = 10f64.powi(precision as i32);
        let threshold = 0.5 / factor;
        let value = -(threshold * (step as f64 / 10_000.0));

        let rendered = number_with(value, NumberOptions::new().precision(precision)).to_string();

        prop_assert_eq!(rendered, "0");
    }

    #[test]
    fn byte_sign_symmetry_holds_for_nonzero_values(value in 1i64..=i64::MAX) {
        let positive = bytes(value).to_string();
        let negative = bytes(-value).to_string();

        prop_assert_eq!(negative, format!("-{positive}"));
    }

    #[test]
    fn duration_output_respects_max_units(
        total_nanos in any::<u64>(),
        max_units in any::<u8>(),
        long_units in any::<bool>(),
    ) {
        let base = DurationOptions::new().max_units(max_units);
        let options = if long_units { base.long_units() } else { base };
        let rendered = duration_with(Duration::from_nanos(total_nanos), options).to_string();
        let unit_count = count_rendered_units(&rendered);

        prop_assert!(!rendered.is_empty());
        prop_assert!(!rendered.contains("  "));
        prop_assert!(!rendered.ends_with(' '));
        prop_assert!(unit_count >= 1);
        // max_units is clamped to 1..=7 (7 is the total number of supported units).
        prop_assert!(unit_count <= usize::from(max_units.clamp(1, 7)));
    }

    #[test]
    fn ago_output_keeps_duration_rendering_as_prefix(
        total_nanos in any::<u64>(),
        max_units in any::<u8>(),
        long_units in any::<bool>(),
    ) {
        let base = DurationOptions::new().max_units(max_units);
        let options = if long_units { base.long_units() } else { base };
        let duration = Duration::from_nanos(total_nanos);

        let duration_rendered = duration_with(duration, options).to_string();
        let ago_rendered = ago_with(duration, options).to_string();

        prop_assert_eq!(ago_rendered, format!("{duration_rendered} ago"));
    }

    #[test]
    fn english_lists_preserve_item_order_and_joining(
        values in prop::collection::vec(0u16..=9999, 0..6),
        serial_comma in any::<bool>(),
    ) {
        let owned_items: Vec<String> = values
            .into_iter()
            .enumerate()
            .map(|(idx, value)| format!("item{idx}_{value}"))
            .collect();
        let item_refs: Vec<&str> = owned_items.iter().map(String::as_str).collect();
        let options = if serial_comma {
            ListOptions::new().serial_comma()
        } else {
            ListOptions::new().no_serial_comma()
        };
        let rendered = list_with(&item_refs, options).to_string();

        let mut cursor = 0;
        for item in &item_refs {
            prop_assert_eq!(rendered.matches(item).count(), 1);

            let offset = rendered[cursor..].find(item);
            prop_assert!(offset.is_some());
            cursor += offset.unwrap() + item.len();
        }

        match item_refs.len() {
            0 => prop_assert!(rendered.is_empty()),
            1 => prop_assert_eq!(rendered, item_refs[0]),
            2 => {
                prop_assert!(!rendered.contains(','));
                prop_assert!(rendered.contains(" and "));
            }
            _ if serial_comma => prop_assert!(rendered.contains(", and ")),
            _ => {
                prop_assert!(!rendered.contains(", and "));
                prop_assert!(rendered.contains(" and "));
            }
        }
    }

    #[test]
    fn number_suffixes_do_not_regress_for_increasing_values(
        start in 0u64..=9_000_000_000_000u64,
        delta in 0u16..=10_000,
    ) {
        let end = start.saturating_add(delta as u64);
        let a = number(start).to_string();
        let b = number(end).to_string();
        let a_idx = number_suffix_index(&a);
        let b_idx = number_suffix_index(&b);

        prop_assert!(a_idx <= b_idx);
    }

    #[test]
    fn byte_suffixes_do_not_regress_for_increasing_values(
        start in 0u64..=u32::MAX as u64,
        delta in 0u16..=10_000,
    ) {
        let end = start.saturating_add(delta as u64);
        let a = bytes(start).to_string();
        let b = bytes(end).to_string();
        let a_idx = byte_suffix_index(&a);
        let b_idx = byte_suffix_index(&b);

        prop_assert!(a_idx <= b_idx);
    }

    #[cfg(feature = "russian")]
    #[test]
    fn russian_number_output_uses_russian_decimal_separator(
        whole in 0u64..=1_000_000_000_000u64,
    ) {
        let raw = whole as f64 + 0.5;
        let rendered = number_with(
            raw,
            NumberOptions::new()
                .locale(humfmt::locale::Russian)
                .separators(true)
                .precision(2),
        )
        .to_string();

        prop_assert!(!rendered.contains('.'));
        prop_assert!(!rendered.contains(".."));
        prop_assert!(!rendered.contains(",,"));
    }

    #[cfg(feature = "polish")]
    #[test]
    fn polish_number_output_uses_polish_decimal_separator(
        whole in 0u64..=1_000_000_000_000u64,
    ) {
        let raw = whole as f64 + 0.5;
        let rendered = number_with(
            raw,
            NumberOptions::new()
                .locale(humfmt::locale::Polish)
                .separators(true)
                .precision(2),
        )
        .to_string();

        prop_assert!(!rendered.contains('.'));
        prop_assert!(!rendered.contains(".."));
        prop_assert!(!rendered.contains(",,"));
    }
}
