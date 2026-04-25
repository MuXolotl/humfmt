use humfmt::{Humanize, NumberOptions};

fn main() {
    println!("{}", humfmt::number(15320));
    println!("{}", humfmt::number(1500000));
    println!("{}", humfmt::number(-12500));
    println!("{}", 1200000000.human_number());
    println!("{}", humfmt::ordinal(21));
    println!("{}", 42.human_ordinal());

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
}
