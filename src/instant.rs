use std::time::Duration;

use crate::{
    Error,
    duration::{Unit, eat_spaces, parse_number, parse_unit},
};

pub(crate) enum RelativeType {
    Subtract,
    Add,
}

#[inline]
pub(crate) fn parse_relative_prefix(input: &[u8]) -> Option<(RelativeType, usize)> {
    match input {
        &[b'+', ..] => Some((RelativeType::Add, 1)),
        &[b'-', ..] => Some((RelativeType::Subtract, 1)),
        _ => None,
    }
}

#[inline]
pub(crate) fn parse_relative_suffix(input: &[u8]) -> Result<RelativeType, Error> {
    if input.is_empty() {
        return Err(Error::EmptyRelativeInput);
    }

    match input {
        &[b'l', b'a', b't', b'e', b'r', ..] => Ok(RelativeType::Add),
        &[b'a', b'g', b'o', ..] => Ok(RelativeType::Subtract),
        _ => Err(Error::InvalidRelativeType),
    }
}

/// Convert an input to a [`Duration`].
///
/// In particular, uses [this](https://github.com/BurntSushi/duration-unit-lookup?tab=readme-ov-file#one-big-match-but-with-prefix-matching) method
/// of parsing time units.
#[inline]
pub(crate) fn parse(
    input: &[u8],
    possible_relative_type: Option<(RelativeType, usize)>,
) -> Result<(Duration, RelativeType), Error> {
    if input.is_empty() {
        return Err(Error::EmptyDurationInput);
    }

    let mut duration = Duration::default();
    let mut current = match possible_relative_type {
        Some((_, offset)) => offset,
        None => 0,
    };

    while current < input.len() {
        current += eat_spaces(&input[current..]);

        let (value, value_length) = match parse_number(&input[current..]) {
            Ok((v, vl)) => (v, vl),
            Err(val_err) => {
                // If current is still 0, then return the value error immediately.
                if current == 0 {
                    return Err(val_err);
                }

                // If there is already a relative type, then return an error.
                if possible_relative_type.is_some() {
                    return Err(Error::MultipleRelativeTypes);
                }

                // It must either end with a relative type or it's invalid.
                match parse_relative_suffix(&input[current..]) {
                    Ok(rel) => {
                        return Ok((duration, rel));
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
            }
        };

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

    let relative_type = match possible_relative_type {
        Some(relative_type) => relative_type.0,
        None => {
            return Err(Error::EmptyRelativeInput);
        }
    };

    Ok((duration, relative_type))
}
