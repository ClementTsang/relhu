# relhu

<div align="center">
  <h1>relhu</h1>
  <p>
    relhu is a library that can parse <i><b>rel</b>ative and/or <b>hu</b>man</i> time duration strings.
  </p>

[<img src="https://img.shields.io/crates/v/relhu.svg?style=flat-square" alt="crates.io link">](https://crates.io/crates/relhu)
[<img src="https://docs.rs/relhu/badge.svg" alt="Documentation">](https://docs.rs/relhu)

</div>

## Usage

```rust
use std::time::{Duration, Instant};

fn main() {
    // Parsing to get a duration.
    assert_eq!(relhu::parse_duration("5s").unwrap(), Duration::from_secs(5));
    assert_eq!(relhu::parse_duration("100 us").unwrap(), Duration::from_micros(100));

    // Parsing to get an instant in the future.
    let now = Instant::now();
    assert_eq!(relhu::parse_with_instant("15m later", now).unwrap(), now + Duration::from_secs(15 * 60));
    assert_eq!(relhu::parse_with_instant("+55ms", now).unwrap(), now + Duration::from_millis(55));

    // Parsing to get an instant in the past.
    let now = Instant::now();
    assert_eq!(relhu::parse_with_instant("20ns ago", now).unwrap(), now - Duration::from_nanos(20));
    assert_eq!(relhu::parse_with_instant("- 5 days", now).unwrap(), now - Duration::from_secs(5 * 60 * 60 * 24));
}
```

## Licensing

relhu is dual-licensed under MIT and Apache-2.0 at your choice.
