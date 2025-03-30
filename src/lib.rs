//! relhu is a library that can parse _**rel**ative **hu**man_ time duration strings.
//!
//! # Examples
//!
//! ```rust
//! ```

mod parser;

use std::time::{Duration, Instant};

pub use parser::Error;

/// Parse `input` as a [`Duration`].
pub fn parse_duration(input: &str) -> Result<Duration, Error> {
    parser::parse(input)
}

/// Parse `input` as an [`Instant`].
pub fn parse_instant(input: &str) -> Result<Instant, Error> {
    parse_with_instant(input, Instant::now())
}

/// Parse `input` as an [`Instant`], based on a provided [`Instant`].
pub fn parse_with_instant(input: &str, now: Instant) -> Result<Instant, Error> {
    let duration = parse_duration(input)?;

    Ok(now + duration)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instant() {
        let now = Instant::now();
        let result = parse_with_instant("5s", now).unwrap();
        assert!(result == now + Duration::from_secs(5));

        let result = parse_with_instant("1h30m", now).unwrap();
        assert!(result == now + Duration::from_secs(90 * 60));

        let result = parse_with_instant("500ms 50us", now).unwrap();
        assert!(result == now + Duration::from_millis(500) + Duration::from_micros(50));
    }

    #[test]
    fn test_parse_instant_errors() {
        assert_eq!(parse_instant("abc").unwrap_err(), Error::InvalidNumber);
        assert_eq!(parse_instant("5x").unwrap_err(), Error::InvalidUnit);
        assert_eq!(parse_instant("").unwrap_err(), Error::EmptyInput);
    }
}
