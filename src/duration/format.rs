use core::fmt;

use super::DurationOptions;

#[derive(Copy, Clone)]
struct Unit {
    nanos: u128,
    short: &'static str,
    singular: &'static str,
    plural: &'static str,
}

const UNITS: [Unit; 7] = [
    Unit {
        nanos: 86_400_000_000_000,
        short: "d",
        singular: "day",
        plural: "days",
    },
    Unit {
        nanos: 3_600_000_000_000,
        short: "h",
        singular: "hour",
        plural: "hours",
    },
    Unit {
        nanos: 60_000_000_000,
        short: "m",
        singular: "minute",
        plural: "minutes",
    },
    Unit {
        nanos: 1_000_000_000,
        short: "s",
        singular: "second",
        plural: "seconds",
    },
    Unit {
        nanos: 1_000_000,
        short: "ms",
        singular: "millisecond",
        plural: "milliseconds",
    },
    Unit {
        nanos: 1_000,
        short: "us",
        singular: "microsecond",
        plural: "microseconds",
    },
    Unit {
        nanos: 1,
        short: "ns",
        singular: "nanosecond",
        plural: "nanoseconds",
    },
];

pub fn format_duration(
    f: &mut fmt::Formatter<'_>,
    value: core::time::Duration,
    options: &DurationOptions,
) -> fmt::Result {
    let mut remaining = value.as_nanos();
    let mut written = 0u8;
    let max_units = options.max_units_value();

    if remaining == 0 {
        return write_unit(f, 0, UNITS[3], options.long_units_value());
    }

    for unit in UNITS {
        if remaining < unit.nanos {
            continue;
        }

        let count = remaining / unit.nanos;
        remaining %= unit.nanos;

        if written != 0 {
            write!(f, " ")?;
        }

        write_unit(f, count, unit, options.long_units_value())?;
        written += 1;

        if written >= max_units {
            break;
        }
    }

    Ok(())
}

fn write_unit(
    f: &mut fmt::Formatter<'_>,
    count: u128,
    unit: Unit,
    long_units: bool,
) -> fmt::Result {
    if long_units {
        let label = if count == 1 {
            unit.singular
        } else {
            unit.plural
        };

        write!(f, "{count} {label}")
    } else {
        write!(f, "{count}{}", unit.short)
    }
}
