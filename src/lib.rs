//! relhu is a library that can parse _**rel**ative and/or **hu**man_ time duration strings.
//!
//! # Examples
//!
//! ```rust
//! use std::time::{Duration, Instant};
//!
//! // Parsing to get a duration.
//! assert_eq!(relhu::parse_duration("5s").unwrap(), Duration::from_secs(5));
//! assert_eq!(relhu::parse_duration("100 us").unwrap(), Duration::from_micros(100));
//!
//! // Parsing to get an instant in the future.
//! let now = Instant::now();
//! assert_eq!(relhu::parse_with_instant("15m later", now).unwrap(), now + Duration::from_secs(15 * 60));
//! assert_eq!(relhu::parse_with_instant("+55ms", now).unwrap(), now + Duration::from_millis(55));
//!
//! // Parsing to get an instant in the past.
//! let now = Instant::now();
//! assert_eq!(relhu::parse_with_instant("20ns ago", now).unwrap(), now - Duration::from_nanos(20));
//! assert_eq!(relhu::parse_with_instant("- 5 days", now).unwrap(), now - Duration::from_secs(5 * 60 * 60 * 24));
//! ```

mod duration;
mod error;
mod instant;

use std::time::{Duration, Instant};

pub use error::Error;
use instant::RelativeType;

/// Parse `input` as a [`Duration`].
///
/// # Examples
/// ```rust
/// use std::time::{Duration, Instant};
///
/// assert_eq!(relhu::parse_duration("5s").unwrap(), Duration::from_secs(5));
/// assert_eq!(relhu::parse_duration("100 us").unwrap(), Duration::from_micros(100));
/// ```
pub fn parse_duration(input: &str) -> Result<Duration, Error> {
    duration::parse(input.as_bytes())
}

fn get_input_and_rel(input: &str) -> Result<(Duration, RelativeType), Error> {
    let input = input.as_bytes();
    let possible_relative_type = instant::parse_relative_prefix(input);

    instant::parse(&input, possible_relative_type)
}

/// Parse `input` as an [`Instant`], based on a provided [`Instant`].
///
/// # Examples
///
/// ```rust
/// use std::time::{Duration, Instant};
///
/// let now = Instant::now();
/// assert_eq!(relhu::parse_with_instant("+55ms", now).unwrap(), now + Duration::from_millis(55));
/// assert_eq!(relhu::parse_with_instant("20ns ago", now).unwrap(), now - Duration::from_nanos(20));
/// ```
pub fn parse_with_instant(input: &str, now: Instant) -> Result<Instant, Error> {
    let (duration, relative_type) = get_input_and_rel(input)?;

    match relative_type {
        instant::RelativeType::Subtract => Ok(now - duration),
        instant::RelativeType::Add => Ok(now + duration),
    }
}

/// Parse `input` as an [`Instant`].
///
/// # Examples
///
/// ```rust
/// use std::time::{Duration, Instant};
///
/// relhu::parse_instant("+55ms").unwrap();
/// ```
pub fn parse_instant(input: &str) -> Result<Instant, Error> {
    parse_with_instant(input, Instant::now())
}

/// Parse `input` as an [`Instant`], based on a provided [`Instant`]. If the input will result in overflow,
/// it will return `None`.
///
/// # Examples
///
/// ```rust
/// use std::time::{Duration, Instant};
///
/// let now = Instant::now();
/// assert_eq!(relhu::checked_parse_with_instant("+55ms", now).unwrap().unwrap(), now + Duration::from_millis(55));
/// assert_eq!(relhu::checked_parse_with_instant("20ns ago", now).unwrap().unwrap(), now - Duration::from_nanos(20));
pub fn checked_parse_with_instant(input: &str, now: Instant) -> Result<Option<Instant>, Error> {
    let (duration, relative_type) = get_input_and_rel(input)?;

    match relative_type {
        instant::RelativeType::Subtract => Ok(now.checked_sub(duration)),
        instant::RelativeType::Add => Ok(now.checked_add(duration)),
    }
}

/// Parse `input` as an [`Instant`]. If the input will result in overflow, it will return `None`.
///
/// # Examples
///
/// ```rust
/// use std::time::{Duration, Instant};
///
/// relhu::checked_parse_instant("+55ms").unwrap();
/// ```
pub fn checked_parse_instant(input: &str) -> Result<Option<Instant>, Error> {
    checked_parse_with_instant(input, Instant::now())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instant() {
        let now = Instant::now();
        let result = parse_with_instant("5s later", now).unwrap();
        assert!(result == now + Duration::from_secs(5));

        let result = parse_with_instant("1h30m later", now).unwrap();
        assert!(result == now + Duration::from_secs(90 * 60));

        let result = parse_with_instant("+55s", now).unwrap();
        assert!(result == now + Duration::from_secs(55));

        let result = parse_with_instant("-50s", now).unwrap();
        assert!(result == now - Duration::from_secs(50));

        let result = parse_with_instant("15m later", now).unwrap();
        assert!(result == now + Duration::from_secs(15 * 60));

        let result = parse_with_instant("500ms 50us ago", now).unwrap();
        assert!(result == now - Duration::from_millis(500) - Duration::from_micros(50));
    }

    #[test]
    fn test_parse_instant_errors() {
        assert_eq!(parse_instant("abc").unwrap_err(), Error::InvalidNumber);
        assert_eq!(parse_instant("5x").unwrap_err(), Error::InvalidUnit);
        assert_eq!(parse_instant("").unwrap_err(), Error::EmptyDurationInput);
        assert_eq!(parse_instant("5h").unwrap_err(), Error::EmptyRelativeInput);
        assert_eq!(
            parse_instant("+5h ago").unwrap_err(),
            Error::MultipleRelativeTypes
        );
    }

    #[cfg(test)]
    #[test]
    fn test_parse_single_units() {
        assert_eq!(parse_duration("5s").unwrap(), Duration::from_secs(5));
        assert_eq!(parse_duration("10m").unwrap(), Duration::from_secs(10 * 60));
        assert_eq!(
            parse_duration("2h").unwrap(),
            Duration::from_secs(2 * 60 * 60)
        );
        assert_eq!(
            parse_duration("1d").unwrap(),
            Duration::from_secs(24 * 60 * 60)
        );
        assert_eq!(parse_duration("500ms").unwrap(), Duration::from_millis(500));
        assert_eq!(parse_duration("100us").unwrap(), Duration::from_micros(100));
        assert_eq!(parse_duration("50ns").unwrap(), Duration::from_nanos(50));
    }

    #[test]
    fn test_parse_full_names() {
        assert_eq!(parse_duration("5seconds").unwrap(), Duration::from_secs(5));
        assert_eq!(
            parse_duration("10minutes").unwrap(),
            Duration::from_secs(10 * 60)
        );
        assert_eq!(
            parse_duration("2hours").unwrap(),
            Duration::from_secs(2 * 60 * 60)
        );
        assert_eq!(
            parse_duration("1day").unwrap(),
            Duration::from_secs(24 * 60 * 60)
        );
        assert_eq!(
            parse_duration("500milliseconds").unwrap(),
            Duration::from_millis(500)
        );
        assert_eq!(
            parse_duration("100microseconds").unwrap(),
            Duration::from_micros(100)
        );
        assert_eq!(
            parse_duration("50nanoseconds").unwrap(),
            Duration::from_nanos(50)
        );
    }

    #[test]
    fn test_parse_multiple_units() {
        assert_eq!(
            parse_duration("1h30m").unwrap(),
            Duration::from_secs(90 * 60)
        );
        assert_eq!(
            parse_duration("2h 45m").unwrap(),
            Duration::from_secs((2 * 60 + 45) * 60)
        );
        assert_eq!(
            parse_duration("1d 12h 30m").unwrap(),
            Duration::from_secs((24 + 12) * 60 * 60 + 30 * 60)
        );
        assert_eq!(
            parse_duration("500ms 50us").unwrap(),
            Duration::from_millis(500) + Duration::from_micros(50)
        );
    }

    #[test]
    fn test_parse_duration_errors() {
        assert_eq!(parse_duration("abc").unwrap_err(), Error::InvalidNumber);
        assert_eq!(parse_duration("5x").unwrap_err(), Error::InvalidUnit);
        assert_eq!(parse_duration("").unwrap_err(), Error::EmptyDurationInput);
    }
}
