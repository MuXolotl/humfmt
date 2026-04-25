use std::time::Duration;

use humfmt::{locale::CustomLocale, BytesOptions, DurationOptions, Humanize, NumberOptions};

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

    let custom_locale = CustomLocale::english()
        .short_suffix(1, "k")
        .separators(',', '.');
    println!(
        "{}",
        humfmt::number_with(15_320, NumberOptions::new().locale(custom_locale))
    );

    #[cfg(feature = "russian")]
    println!(
        "{}",
        humfmt::number_with(15_320, NumberOptions::new().locale(humfmt::locale::Russian))
    );

    #[cfg(feature = "polish")]
    println!(
        "{}",
        humfmt::number_with(15_320, NumberOptions::new().locale(humfmt::locale::Polish))
    );
}
