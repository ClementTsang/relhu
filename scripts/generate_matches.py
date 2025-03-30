#!/usr/bin/env python3

"""
A simple script that generates the matches for the parser.

Yes I realize that it's kinda silly to generate a Rust match using a Python script,
but it works and I don't want to write this out by hand. Also I don't want to use
macros to do it since that might be slow.
"""

from dataclasses import dataclass


@dataclass
class TimeUnit:
    unit: str
    aliases: list[str]


def generate_matches():
    nanoseconds = TimeUnit(
        "Nanoseconds", ["nanosecond", "ns", "nanos", "nsecs", "nsec"]
    )
    microseconds = TimeUnit(
        "Microseconds", ["microsecond", "us", "μs", "µs", "usecs", "usec"]
    )
    milliseconds = TimeUnit("Milliseconds", ["millisecond", "ms", "msecs", "msec"])
    seconds = TimeUnit("Seconds", ["second", "s", "secs", "sec"])
    minutes = TimeUnit("Minutes", ["minute", "m", "mins", "min"])
    hours = TimeUnit("Hours", ["hour", "h", "hrs", "hr"])
    days = TimeUnit("Days", ["day", "d"])
    # weeks = TimeUnit("Weeks", ["week", "w"])
    # months = TimeUnit("Months", ["month", "mo", "M"])
    # years = TimeUnit("Years", ["year", "y", "yrs", "yr"])

    units = [
        nanoseconds,
        microseconds,
        milliseconds,
        seconds,
        minutes,
        hours,
        days,
        # weeks,
        # months,
        # years,
    ]

    print("match input {")

    work = []
    for unit in units:
        for to_match in [unit.unit.lower(), *unit.aliases]:
            work.append((unit, to_match))

    work.sort(key=lambda x: len(x[1]), reverse=True)

    for unit, to_match in work:
        chars = [c for c in to_match]

        final_byte_list = []
        for c in chars:
            if not c.isascii():
                encoding = c.encode()
                for e in encoding:
                    final_byte_list.append(f"b'\\x{format(e, '02x')}'")
            else:
                final_byte_list.append(f"b'{c}'")
        match = ", ".join(final_byte_list)
        print(f"&[{match}, ..] => {{")
        print(f"Ok((Unit::{unit.unit}, {len(final_byte_list)}))")
        print("}")

    print("_ => {")
    print("Err(Error::InvalidUnit)")
    print("}")
    print("}")


if __name__ == "__main__":
    generate_matches()
