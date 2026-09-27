//! Exact decimal digits of `f64` values.
//!
//! Scaling a binary float with `f64` arithmetic rounds twice: once in the
//! multiply, once in the print. `0.15 * 10.0` is `1.5000000000000002`, so a
//! value a caller typed as a short decimal can round the wrong way. The digits
//! here come from `core`'s float formatting, which renders the exact decimal
//! expansion, and the decimal point is then moved over that digit string
//! instead of over the binary value.

use core::fmt;
use core::fmt::Write;

use crate::rounding::carry_after_truncation;
use crate::RoundingMode;

/// Digit storage. The integer part of `f64::MAX` is 309 digits, and the widest
/// guarded fraction below the decimal point stays inside the remainder.
const CAPACITY: usize = 384;

/// Places printed below the digit that rounding reads.
///
/// `core` rounds its own last printed digit, and the carry out of that place
/// reaches the rounding position only by crossing a run of nines in the exact
/// expansion. The guard puts thirty-two places between the two, past the
/// sixteen or so nines that a binary float can hold in a row: a run of `n`
/// nines needs the value to sit within a relative `10^-n` of a decimal
/// boundary, and neighbouring `f64` values are never closer than `2^-53` of
/// the magnitude.
const GUARD: usize = 32;

/// Writes into a fixed buffer and reports overflow through `fmt::Error`.
struct Sink<'a> {
    buf: &'a mut [u8],
    len: usize,
}

impl fmt::Write for Sink<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();

        if self.len + bytes.len() > self.buf.len() {
            return Err(fmt::Error);
        }

        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();

        Ok(())
    }
}

/// Exact decimal digits of a non-negative finite `f64`.
///
/// The stored digits are what `core` printed, most significant first, with the
/// decimal point removed: the value is `0.<stored digits> * 10^point`. Places
/// below the stored digits are zero unless `end` says the expansion reaches
/// further, which is what lets rounding tell a true zero remainder from one
/// that is merely out of range.
#[derive(Clone, Copy)]
pub(crate) struct Digits {
    buf: [u8; CAPACITY],
    /// Number of stored digits; `buf[..used]` are ASCII digits.
    used: usize,
    /// Digits stored before the decimal point, at least one.
    point: i32,
    /// Decimal place of the last nonzero digit of the exact expansion.
    ///
    /// `None` for zero, whose expansion has no nonzero digit at all. The
    /// expansion of `m / 2^d` is `m * 5^d / 10^d`, whose last digit is a `5`,
    /// so the place follows from the binary exponent without computing digits.
    end: Option<i32>,
}

impl Digits {
    /// Prints `abs` so that `place` can be rounded and the digit below it read.
    pub(crate) fn new(abs: f64, place: i32) -> Self {
        Self::covering(abs, place, place - 1)
    }

    /// Prints `abs` so that `place` can be rounded and `readable` is stored.
    ///
    /// Callers that only know where the rounding position lands after seeing
    /// the leading digit, as significant-digit rounding does, pass the place
    /// that digit can occupy as `readable`.
    pub(crate) fn covering(abs: f64, place: i32, readable: i32) -> Self {
        debug_assert!(abs.is_finite() && abs >= 0.0);

        // The deepest place the caller reads, and the guard below it. A value
        // that rounds above the decimal point still prints a short fraction,
        // so that the guard also covers the digits of the integer part.
        let deepest = place.saturating_sub(1).min(readable);
        let below = if deepest < 0 { (-deepest) as usize } else { 0 };
        let depth = (below + GUARD).min(CAPACITY - 2);

        Self::print(abs, depth)
    }

    /// Rounds the value at `place`, keeping the digits at and above it.
    ///
    /// `HalfUp` needs the digit below the cut, because a tie rounds up as well.
    /// `Floor` and `Ceil` need to know whether anything at all was dropped,
    /// which `end` answers exactly.
    pub(crate) fn round_at(&mut self, place: i32, rounding: RoundingMode, is_negative: bool) {
        let keep = self.point - place;
        let below = self.digit_at(place - 1);
        let dropped_anything = self.end.is_some_and(|end| end < place);

        let carry = carry_after_truncation(below >= b'5', dropped_anything, rounding, is_negative);

        if keep <= 0 {
            self.used = 0;
            self.end = None;

            // The value was below half of the kept place and rounded to nothing
            // unless the carry put a single one there.
            if carry {
                self.buf[0] = b'1';
                self.used = 1;
                self.point = place + 1;
                self.end = Some(place);
            }

            return;
        }

        self.used = (keep as usize).min(self.used);

        if carry {
            self.increment();
        }

        // The stored digits are now the whole value, so the expansion ends at
        // the last nonzero one.
        self.end = self.last_nonzero();
    }

    /// Divides the value by `10^shift`, moving the decimal point left.
    ///
    /// A negative shift multiplies instead, which is how the percent formatter
    /// applies its hundred. Every stored digit keeps its relative position, so
    /// the place of the last nonzero digit moves with the point.
    pub(crate) fn shift(&mut self, shift: i32) {
        self.point -= shift;

        if let Some(end) = self.end.as_mut() {
            *end -= shift;
        }
    }

    /// Decimal place of the first nonzero stored digit, if there is one.
    pub(crate) fn leading_place(&self) -> Option<i32> {
        for idx in 0..self.used {
            if self.buf[idx] != b'0' {
                return Some(self.point - 1 - idx as i32);
            }
        }

        None
    }

    /// Returns `true` when every stored digit is zero.
    pub(crate) fn is_zero(&self) -> bool {
        self.leading_place().is_none()
    }

    /// Writes the value with at most `decimals` fraction digits.
    ///
    /// Fraction zeros past the last nonzero digit are dropped unless
    /// `fixed_precision` asks for exactly `decimals` of them.
    pub(crate) fn write<W: fmt::Write + ?Sized>(
        &self,
        f: &mut W,
        decimals: usize,
        fixed_precision: bool,
        group: bool,
        group_separator: char,
        decimal_separator: char,
    ) -> fmt::Result {
        // The integer part starts at the leading digit, which a scale-up can
        // leave below the decimal point, and a value that rounded to zero has
        // no digits of its own.
        match self.leading_place() {
            Some(leading) => self.write_integer(f, leading, group, group_separator)?,
            None => f.write_str("0")?,
        }

        let fraction_digits = if fixed_precision {
            decimals
        } else {
            let mut len = decimals;

            while len > 0 && self.digit_at(-(len as i32)) == b'0' {
                len -= 1;
            }

            len
        };

        if fraction_digits == 0 {
            return Ok(());
        }

        f.write_char(decimal_separator)?;

        for place in 1..=fraction_digits {
            f.write_char(char::from(self.digit_at(-(place as i32))))?;
        }

        Ok(())
    }

    /// Writes the digits from `leading` down to the decimal point.
    ///
    /// Places the buffer does not cover read as zeros, which is how a rounded
    /// carry keeps the width of its unit. A leading digit below the decimal
    /// point leaves a single zero in front of the fraction.
    fn write_integer<W: fmt::Write + ?Sized>(
        &self,
        f: &mut W,
        leading: i32,
        group: bool,
        group_separator: char,
    ) -> fmt::Result {
        if leading < 0 {
            return f.write_str("0");
        }

        let digits = leading as usize + 1;
        let mut written = 0;

        while written < digits {
            if written != 0 && group && (digits - written) % 3 == 0 {
                f.write_char(group_separator)?;
            }

            f.write_char(char::from(self.digit_at(leading - written as i32)))?;
            written += 1;
        }

        Ok(())
    }

    /// Digit at a decimal place, zero outside the stored range.
    fn digit_at(&self, place: i32) -> u8 {
        let idx = self.point - 1 - place;

        if idx < 0 || idx as usize >= self.used {
            b'0'
        } else {
            self.buf[idx as usize]
        }
    }

    /// Place of the deepest nonzero stored digit.
    fn last_nonzero(&self) -> Option<i32> {
        for idx in (0..self.used).rev() {
            if self.buf[idx] != b'0' {
                return Some(self.point - 1 - idx as i32);
            }
        }

        None
    }

    /// Adds one to the least significant stored digit.
    fn increment(&mut self) {
        let mut idx = self.used;

        while idx > 0 {
            idx -= 1;

            if self.buf[idx] != b'9' {
                self.buf[idx] += 1;
                return;
            }

            self.buf[idx] = b'0';
        }

        // Every digit was a nine; the carry adds a new leading one.
        self.point += 1;
        self.used = 1;
        self.buf[0] = b'1';
    }

    fn print(abs: f64, depth: usize) -> Self {
        // Zero has no digits, and a depth sized for the caller's rounding place
        // can be the whole buffer.
        if abs == 0.0 {
            return Self {
                buf: [0u8; CAPACITY],
                used: 0,
                point: 0,
                end: None,
            };
        }

        let mut buf = [0u8; CAPACITY];
        let mut sink = Sink {
            buf: &mut buf,
            len: 0,
        };

        // A rendering only fails when the integer part fills the buffer, and
        // such a value has no fraction digits to lose, so halving the depth
        // reaches a rendering that fits without dropping anything.
        let mut depth = depth;

        while depth > 0 && write!(sink, "{abs:.depth$}").is_err() {
            sink.len = 0;
            depth /= 2;
        }

        if depth == 0 {
            sink.len = 0;

            // An integer part of an `f64` is at most 309 digits, so this fits.
            let _ = write!(sink, "{abs:.0}");
        }

        let text = core::str::from_utf8(&sink.buf[..sink.len]).unwrap_or("0");
        let mut digits = Self::parse(text);
        digits.end = expansion_end(abs);

        digits
    }

    /// Loads digits from a `core` float rendering, which always has at least
    /// one integer digit.
    fn parse(text: &str) -> Self {
        debug_assert!(!text.is_empty());

        let mut digits = Self {
            buf: [0u8; CAPACITY],
            used: 0,
            point: 0,
            end: None,
        };

        for &byte in text.as_bytes() {
            if byte == b'.' {
                digits.point = digits.used as i32;
            } else {
                debug_assert!(byte.is_ascii_digit());

                digits.buf[digits.used] = byte;
                digits.used += 1;
            }
        }

        if digits.point == 0 {
            digits.point = digits.used as i32;
        }

        // `core` prints one `0` before the point for values below one. Storing
        // it would turn into a visible zero when the point is later scaled up,
        // so drop it: the places of the remaining digits do not change.
        if digits.point == 1 && digits.used > 1 && digits.buf[0] == b'0' {
            digits.buf.copy_within(1..digits.used, 0);
            digits.used -= 1;
            digits.point = 0;
        }

        digits
    }
}

/// Decimal place of the last nonzero digit in the exact expansion of `abs`.
///
/// `None` for zero. Only the bit pattern is read: `abs` is `m * 2^e` with `m`
/// odd, and for a negative `e` the expansion is `m * 5^-e / 10^-e`, whose last
/// nonzero digit sits at place `e`. For a non-negative `e` the value is an
/// integer whose last nonzero digit moves up by one place for every factor of
/// ten it carries.
fn expansion_end(abs: f64) -> Option<i32> {
    if abs == 0.0 {
        return None;
    }

    let bits = abs.to_bits();
    let biased = ((bits >> 52) & 0x7FF) as i32;
    let fraction = bits & ((1u64 << 52) - 1);

    let (mantissa, exponent) = if biased == 0 {
        (fraction, -1074)
    } else {
        (fraction | (1u64 << 52), biased - 1075)
    };

    // Only the odd part decides where the expansion ends; the powers of two
    // below it move the decimal point instead.
    let trailing_zeros = mantissa.trailing_zeros();
    let odd = mantissa >> trailing_zeros;
    let exponent = exponent + trailing_zeros as i32;

    Some(if exponent >= 0 {
        exponent.min(count_fives(odd))
    } else {
        exponent
    })
}

/// Number of times `5` divides `value`.
fn count_fives(mut value: u64) -> i32 {
    let mut count = 0;

    while value != 0 && value % 5 == 0 {
        value /= 5;
        count += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Renders the digits into a stack buffer and compares them, so that these
    /// tests need neither `std` nor an allocator.
    fn assert_rendered(digits: &Digits, decimals: usize, fixed_precision: bool, expected: &str) {
        let mut buf = [0u8; 64];
        let mut sink = Sink {
            buf: &mut buf,
            len: 0,
        };

        digits
            .write(&mut sink, decimals, fixed_precision, false, ',', '.')
            .expect("the test buffer is large enough");

        let text = core::str::from_utf8(&sink.buf[..sink.len]).expect("the sink holds ASCII");

        assert_eq!(text, expected);
    }

    #[test]
    fn expansion_ends_where_the_last_nonzero_digit_sits() {
        assert_eq!(expansion_end(0.0), None);
        assert_eq!(expansion_end(1.0), Some(0));
        // `0.5` is exactly `2^-1`, and `1.5` is `3 * 2^-1`.
        assert_eq!(expansion_end(0.5), Some(-1));
        assert_eq!(expansion_end(1.5), Some(-1));
        // `0.2` is `3602879701896397 * 2^-54`.
        assert_eq!(expansion_end(0.2), Some(-54));
        // `1000` is `125 * 2^3`, and `125` carries three factors of five.
        assert_eq!(expansion_end(1_000.0), Some(3));
    }

    #[test]
    fn printing_keeps_the_digits_below_the_rounding_place() {
        // The exact value of `0.145` is `0.14499999999999999000798…`, so the
        // digit below the second place is a four and not a five.
        let digits = Digits::new(0.145, -2);

        assert_eq!(digits.leading_place(), Some(-1));
        assert_eq!(digits.digit_at(-1), b'1');
        assert_eq!(digits.digit_at(-2), b'4');
        assert_eq!(digits.digit_at(-3), b'4');
        assert_eq!(digits.end, Some(-55));
    }

    #[test]
    fn rounding_reads_the_digit_below_the_cut() {
        let mut below_five = Digits::new(0.145, -2);
        below_five.round_at(-2, RoundingMode::HalfUp, false);
        assert_eq!(below_five.leading_place(), Some(-1));
        assert_eq!(below_five.last_nonzero(), Some(-2));

        let mut at_five = Digits::new(0.1451, -2);
        at_five.round_at(-2, RoundingMode::HalfUp, false);
        assert_eq!(at_five.digit_at(-1), b'1');
        assert_eq!(at_five.digit_at(-2), b'5');
    }

    #[test]
    fn rounding_carries_out_of_every_kept_nine() {
        // `0.999` at one decimal place is `1`.
        let mut carried = Digits::new(0.999, -1);
        carried.round_at(-1, RoundingMode::HalfUp, false);
        assert_eq!(carried.leading_place(), Some(0));
        assert_eq!(carried.digit_at(0), b'1');

        // A carry out of digits above the decimal point opens a place above
        // them: `999.95` at one decimal place is `1000`.
        let mut integer = Digits::new(999.95, -1);
        integer.round_at(-1, RoundingMode::HalfUp, false);
        assert_eq!(integer.leading_place(), Some(3));
        assert_eq!(integer.digit_at(3), b'1');
        assert_eq!(integer.digit_at(2), b'0');
    }

    #[test]
    fn rounding_above_the_point_fills_one_place_at_most() {
        // A positive remainder below one never floors to anything, and its
        // ceiling is the single one the drop below one half cannot reach.
        let mut floored = Digits::new(0.45, 0);
        floored.round_at(0, RoundingMode::Floor, false);
        assert!(floored.is_zero());
        assert_eq!(floored.last_nonzero(), None);

        let mut ceiled = Digits::new(0.45, 0);
        ceiled.round_at(0, RoundingMode::Ceil, false);
        assert_eq!(ceiled.leading_place(), Some(0));
        assert_eq!(ceiled.end, Some(0));

        // The same drop far below the decimal point, reached after a scale-up
        // that leaves the digits behind the point.
        let mut scaled_up = Digits::new(1e-300, 0);
        scaled_up.shift(-2);
        scaled_up.round_at(0, RoundingMode::Ceil, false);
        assert_eq!(scaled_up.digit_at(1), b'0');
        assert_eq!(scaled_up.digit_at(0), b'1');
        assert_rendered(&scaled_up, 0, false, "1");
    }

    #[test]
    fn shifting_moves_the_point_and_the_expansion() {
        // The compact scale divides by a thousand.
        let mut divided = Digits::new(1_000.0, -1);
        divided.shift(3);
        assert_eq!(divided.leading_place(), Some(0));
        assert_eq!(divided.end, Some(0));

        // The percent scale multiplies by a hundred, which lifts the leading
        // digit of `0.05` into the ones place and moves the end of the
        // expansion, which reaches past the printed digits, up by two places.
        let mut multiplied = Digits::new(0.05, -2);
        assert_eq!(multiplied.leading_place(), Some(-2));
        assert_eq!(multiplied.end, Some(-56));
        multiplied.shift(-2);
        assert_eq!(multiplied.leading_place(), Some(0));
        assert_eq!(multiplied.end, Some(-54));
    }

    #[test]
    fn writing_pads_or_drops_fraction_zeros() {
        let mut half = Digits::new(0.5, -3);
        half.round_at(-3, RoundingMode::HalfUp, false);
        assert_rendered(&half, 3, false, "0.5");
        assert_rendered(&half, 3, true, "0.500");

        let mut zeroed = Digits::new(0.4, 0);
        zeroed.round_at(0, RoundingMode::HalfUp, false);
        assert_rendered(&zeroed, 0, false, "0");
        assert_rendered(&zeroed, 2, true, "0.00");
    }
}
