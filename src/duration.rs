//! Parser inspired by the work done by burntsushi in [this](https://github.com/BurntSushi/duration-unit-lookup)
//! repository.

use std::time::Duration;

use crate::Error;

/// Convert an input to a [`Duration`].
///
/// In particular, uses [this](https://github.com/BurntSushi/duration-unit-lookup?tab=readme-ov-file#one-big-match-but-with-prefix-matching) method
/// of parsing time units.
#[inline]
pub(crate) fn parse(input: &[u8]) -> Result<Duration, Error> {
    if input.is_empty() {
        return Err(Error::EmptyDurationInput);
    }

    let mut duration = Duration::default();
    let mut current = 0;

    while current < input.len() {
        current += eat_spaces(&input[current..]);

        let (value, value_length) = parse_number(&input[current..])?;
        current += value_length;

        current += eat_spaces(&input[current..]);

        let (unit, unit_length) = parse_unit(&input[current..])?;
        current += unit_length;

        duration += match unit {
            Unit::Nanoseconds => Duration::from_nanos(value),
            Unit::Microseconds => Duration::from_micros(value),
            Unit::Milliseconds => Duration::from_millis(value),
            Unit::Seconds => Duration::from_secs(value),
            Unit::Minutes => Duration::from_secs(value * 60),
            Unit::Hours => Duration::from_secs(value * 60 * 60),
            Unit::Days => Duration::from_secs(value * 60 * 60 * 24),
        }
    }

    Ok(duration)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Unit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    // Weeks,
    // Months,
    // Years,
}

#[inline]
pub(crate) fn eat_spaces(input: &[u8]) -> usize {
    let mut length = 0;

    while length < input.len() && input[length].is_ascii_whitespace() {
        length += 1;
    }

    length
}

#[inline]
pub(crate) fn parse_number(input: &[u8]) -> Result<(u64, usize), Error> {
    let mut number: u64 = 0;
    let mut length = 0;

    while length < input.len() && input[length].is_ascii_digit() {
        number = number * 10 + u64::from(input[length] - b'0');
        length += 1;
    }

    if length == 0 {
        Err(Error::InvalidNumber)
    } else {
        Ok((number, length))
    }
}

#[inline]
pub(crate) fn parse_unit(input: &[u8]) -> Result<(Unit, usize), Error> {
    match input {
        &[
            b'm',
            b'i',
            b'c',
            b'r',
            b'o',
            b's',
            b'e',
            b'c',
            b'o',
            b'n',
            b'd',
            b's',
            ..,
        ] => Ok((Unit::Microseconds, 12)),
        &[
            b'm',
            b'i',
            b'l',
            b'l',
            b'i',
            b's',
            b'e',
            b'c',
            b'o',
            b'n',
            b'd',
            b's',
            ..,
        ] => Ok((Unit::Milliseconds, 12)),
        &[
            b'n',
            b'a',
            b'n',
            b'o',
            b's',
            b'e',
            b'c',
            b'o',
            b'n',
            b'd',
            b's',
            ..,
        ] => Ok((Unit::Nanoseconds, 11)),
        &[
            b'm',
            b'i',
            b'c',
            b'r',
            b'o',
            b's',
            b'e',
            b'c',
            b'o',
            b'n',
            b'd',
            ..,
        ] => Ok((Unit::Microseconds, 11)),
        &[
            b'm',
            b'i',
            b'l',
            b'l',
            b'i',
            b's',
            b'e',
            b'c',
            b'o',
            b'n',
            b'd',
            ..,
        ] => Ok((Unit::Milliseconds, 11)),
        &[
            b'n',
            b'a',
            b'n',
            b'o',
            b's',
            b'e',
            b'c',
            b'o',
            b'n',
            b'd',
            ..,
        ] => Ok((Unit::Nanoseconds, 10)),
        &[b's', b'e', b'c', b'o', b'n', b'd', b's', ..] => Ok((Unit::Seconds, 7)),
        &[b'm', b'i', b'n', b'u', b't', b'e', b's', ..] => Ok((Unit::Minutes, 7)),
        &[b's', b'e', b'c', b'o', b'n', b'd', ..] => Ok((Unit::Seconds, 6)),
        &[b'm', b'i', b'n', b'u', b't', b'e', ..] => Ok((Unit::Minutes, 6)),
        &[b'n', b'a', b'n', b'o', b's', ..] => Ok((Unit::Nanoseconds, 5)),
        &[b'n', b's', b'e', b'c', b's', ..] => Ok((Unit::Nanoseconds, 5)),
        &[b'u', b's', b'e', b'c', b's', ..] => Ok((Unit::Microseconds, 5)),
        &[b'm', b's', b'e', b'c', b's', ..] => Ok((Unit::Milliseconds, 5)),
        &[b'h', b'o', b'u', b'r', b's', ..] => Ok((Unit::Hours, 5)),
        &[b'n', b's', b'e', b'c', ..] => Ok((Unit::Nanoseconds, 4)),
        &[b'u', b's', b'e', b'c', ..] => Ok((Unit::Microseconds, 4)),
        &[b'm', b's', b'e', b'c', ..] => Ok((Unit::Milliseconds, 4)),
        &[b's', b'e', b'c', b's', ..] => Ok((Unit::Seconds, 4)),
        &[b'm', b'i', b'n', b's', ..] => Ok((Unit::Minutes, 4)),
        &[b'h', b'o', b'u', b'r', ..] => Ok((Unit::Hours, 4)),
        &[b'd', b'a', b'y', b's', ..] => Ok((Unit::Days, 4)),
        &[b's', b'e', b'c', ..] => Ok((Unit::Seconds, 3)),
        &[b'm', b'i', b'n', ..] => Ok((Unit::Minutes, 3)),
        &[b'h', b'r', b's', ..] => Ok((Unit::Hours, 3)),
        &[b'd', b'a', b'y', ..] => Ok((Unit::Days, 3)),
        &[b'n', b's', ..] => Ok((Unit::Nanoseconds, 2)),
        &[b'u', b's', ..] => Ok((Unit::Microseconds, 2)),
        &[b'\xce', b'\xbc', b's', ..] => Ok((Unit::Microseconds, 3)),
        &[b'\xc2', b'\xb5', b's', ..] => Ok((Unit::Microseconds, 3)),
        &[b'm', b's', ..] => Ok((Unit::Milliseconds, 2)),
        &[b'h', b'r', ..] => Ok((Unit::Hours, 2)),
        &[b's', ..] => Ok((Unit::Seconds, 1)),
        &[b'm', ..] => Ok((Unit::Minutes, 1)),
        &[b'h', ..] => Ok((Unit::Hours, 1)),
        &[b'd', ..] => Ok((Unit::Days, 1)),
        _ => Err(Error::InvalidUnit),
    }
}
