#!/usr/bin/env python3

"""
A simple script that generates the matches for the parser.

Yes I realize that it's kinda silly to generate a Rust match using a Python script,
but it works and I don't want to write this out by hand. Also I don't want to use
macros to do it since that might be slow.
"""

from dataclasses import dataclass


@dataclass
class RelativeType:
    ty: str
    aliases: list[str]


def generate_matches():
    subtract = RelativeType("Subtract", ["ago"])
    add = RelativeType("Add", ["later"])

    types = [
        subtract,
        add,
    ]

    print("match input {")

    work = []
    for ty in types:
        for to_match in ty.aliases:
            work.append((ty, to_match))

    work.sort(key=lambda x: len(x[1]), reverse=True)

    for ty, to_match in work:
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
        print(f"Ok(RelativeType::{ty.ty})")
        print("}")

    print("_ => {")
    print("Err(Error::InvalidRelativeType)")
    print("}")
    print("}")


if __name__ == "__main__":
    generate_matches()
