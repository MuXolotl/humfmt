use std::time::Duration;

use humfmt::{BytesOptions, DurationOptions, Humanize, ListOptions, NumberOptions};

fn main() {
    println!("{}", humfmt::number(15320));
    println!("{}", humfmt::number(1500000));
    println!("{}", humfmt::number(-12500));
    println!("{}", 1200000000.human_number());
    println!("{}", humfmt::bytes(1536));
    println!(
        "{}",
        1536_u64.human_bytes_with(BytesOptions::new().binary())
    );
    println!("{}", humfmt::ordinal(21));
    println!("{}", 42.human_ordinal());
    println!("{}", humfmt::duration(Duration::from_secs(3661)));
    println!(
        "{}",
        Duration::from_millis(1500).human_duration_with(DurationOptions::new().long_units())
    );
    println!("{}", humfmt::ago(Duration::from_secs(90)));
    println!(
        "{}",
        Duration::from_secs(3665).human_ago_with(DurationOptions::new().max_units(3))
    );
    println!("{}", humfmt::list(&["red", "green", "blue"]));
    println!(
        "{}",
        humfmt::list_with(
            &["red", "green", "blue"],
            ListOptions::new().no_serial_comma()
        )
    );

    println!("{}", humfmt::number(999_949));
    println!("{}", humfmt::number(999_950));
    println!("{}", humfmt::number(999_999));
    println!("{}", humfmt::number(1_000_000));

    println!(
        "{}",
        1_234_567.human_number_with(NumberOptions::new().precision(4))
    );

    println!(
        "{}",
        15_320.human_number_with(NumberOptions::new().long_units())
    );

    // Custom separators: replaces the old custom-locale demonstration.
    println!(
        "{}",
        humfmt::number_with(
            15_320,
            NumberOptions::new()
                .decimal_separator(',')
                .group_separator('.')
        )
    );
    println!(
        "{}",
        humfmt::list_with(
            &["red", "green", "blue"],
            ListOptions::new().conjunction("plus").no_serial_comma()
        )
    );

    // Percentage formatting.
    println!("{}", humfmt::percent(0.423));
    println!(
        "{}",
        0.5_f64.human_percent_with(
            humfmt::PercentOptions::new()
                .precision(2)
                .fixed_precision(true)
        )
    );
}
