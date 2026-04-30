use core::fmt;

use crate::locale::{DurationUnit, Locale};

use super::DurationOptions;

#[derive(Copy, Clone)]
struct Unit {
    nanos: u128,
    kind: DurationUnit,
}

const UNITS: [Unit; 7] = [
    Unit {
        nanos: 86_400_000_000_000,
        kind: DurationUnit::Day,
    },
    Unit {
        nanos: 3_600_000_000_000,
        kind: DurationUnit::Hour,
    },
    Unit {
        nanos: 60_000_000_000,
        kind: DurationUnit::Minute,
    },
    Unit {
        nanos: 1_000_000_000,
        kind: DurationUnit::Second,
    },
    Unit {
        nanos: 1_000_000,
        kind: DurationUnit::Millisecond,
    },
    Unit {
        nanos: 1_000,
        kind: DurationUnit::Microsecond,
    },
    Unit {
        nanos: 1,
        kind: DurationUnit::Nanosecond,
    },
];

pub fn format_duration<L: Locale>(
    f: &mut fmt::Formatter<'_>,
    value: core::time::Duration,
    options: &DurationOptions<L>,
) -> fmt::Result {
    let mut remaining = value.as_nanos();
    let mut written = 0u8;
    let max_units = options.max_units;
    let locale = &options.locale;

    if remaining == 0 {
        return write_unit(f, 0, UNITS[3], options.long_units, locale);
    }

    for unit in UNITS {
        if remaining < unit.nanos {
            continue;
        }

        let count = remaining / unit.nanos;
        remaining %= unit.nanos;

        if written != 0 {
            f.write_str(" ")?;
        }

        write_unit(f, count, unit, options.long_units, locale)?;
        written += 1;

        if written >= max_units {
            break;
        }
    }

    Ok(())
}

fn write_unit<L: Locale>(
    f: &mut fmt::Formatter<'_>,
    count: u128,
    unit: Unit,
    long_units: bool,
    locale: &L,
) -> fmt::Result {
    let label = locale.duration_unit(unit.kind, count, long_units);

    if long_units {
        write!(f, "{count} {label}")
    } else {
        write!(f, "{count}{label}")
    }
}
