use core::fmt;

use super::DurationOptions;

#[derive(Copy, Clone)]
struct Unit {
    nanos: u128,
    short: &'static str,
    long_singular: &'static str,
    long_plural: &'static str,
}

const UNITS: [Unit; 7] = [
    Unit {
        nanos: 86_400_000_000_000,
        short: "d",
        long_singular: "day",
        long_plural: "days",
    },
    Unit {
        nanos: 3_600_000_000_000,
        short: "h",
        long_singular: "hour",
        long_plural: "hours",
    },
    Unit {
        nanos: 60_000_000_000,
        short: "m",
        long_singular: "minute",
        long_plural: "minutes",
    },
    Unit {
        nanos: 1_000_000_000,
        short: "s",
        long_singular: "second",
        long_plural: "seconds",
    },
    Unit {
        nanos: 1_000_000,
        short: "ms",
        long_singular: "millisecond",
        long_plural: "milliseconds",
    },
    Unit {
        nanos: 1_000,
        short: "us",
        long_singular: "microsecond",
        long_plural: "microseconds",
    },
    Unit {
        nanos: 1,
        short: "ns",
        long_singular: "nanosecond",
        long_plural: "nanoseconds",
    },
];

// Index of the "second" unit, used as the placeholder for zero durations.
const SECOND_UNIT_IDX: usize = 3;

pub fn format_duration(
    f: &mut fmt::Formatter<'_>,
    value: core::time::Duration,
    options: &DurationOptions,
) -> fmt::Result {
    let mut remaining = value.as_nanos();
    let mut written = 0u8;
    let max_units = options.max_units;

    if remaining == 0 {
        return write_unit(f, 0, &UNITS[SECOND_UNIT_IDX], options.long_units);
    }

    for unit in &UNITS {
        if remaining < unit.nanos {
            continue;
        }

        let count = remaining / unit.nanos;
        remaining %= unit.nanos;

        if written != 0 {
            f.write_str(" ")?;
        }

        write_unit(f, count, unit, options.long_units)?;
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
    unit: &Unit,
    long_units: bool,
) -> fmt::Result {
    if long_units {
        let label = if count == 1 {
            unit.long_singular
        } else {
            unit.long_plural
        };
        write!(f, "{count} {label}")
    } else {
        let short = unit.short;
        write!(f, "{count}{short}")
    }
}
