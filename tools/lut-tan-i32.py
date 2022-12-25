#!/usr/bin/env python

"""
This script generates a Rust lookup table for tangent using fixed-point i32 values.

Author:     Oliver Rockstedt <info@sourcebox.de>
License:    MIT
"""

# System libraries.
import os
import math
import string
import subprocess


# Number of table entries.
ENTRIES = 32

# Total number of bits.
TOTAL_BITS = 32

# Number of fractional bits.
FRAC_BITS = 16

# File to be generated, path is relative to this file.
OUTPUT_FILE = "../src/tan.rs"

# Template used for the generated file.
FILE_TEMPLATE = """
//! Lookup table for fixed-point tangent function.
//!
//! This file is auto-generated. Don't edit!
//! Build script: $script

/// Number of table entries.
pub(crate) const TAN_LUT_LENGTH: usize = $entries;

/// Number of fractional bits used in table values.
pub(crate) const TAN_LUT_FRAC_BITS: u8 = $frac_bits;

/// Table data in the range 0..PI/2.
pub(crate) const TAN_LUT: [i32; $entries] = [
    $data
];
"""


def generate_data():
    """
    Calculate the table data and return it.
    """
    data = []

    for n in range(ENTRIES):
        x = float(n) / float(ENTRIES) * math.pi / 2
        y = math.tan(x)
        v = int(y * (1 << FRAC_BITS))
        v = min(v, (2 ** (TOTAL_BITS - 1)) - 1)
        data.append(v)

    return data


def chunks(lst, n):
    """
    Yield successive n-sized chunks from lst.
    """
    for i in range(0, len(lst), n):
        yield lst[i:i + n]


def format_data(data):
    """
    Return data formatted as text.
    """
    lines = []

    for chunk in chunks(data, 8):
        s = [str(i) for i in chunk]
        line = ", ".join(s)
        lines.append(line)

    return ",\n    ".join(lines)


if __name__ == '__main__':
    # Generate and format the data.
    data = generate_data()
    template = string.Template(FILE_TEMPLATE)
    output = template.substitute(entries=ENTRIES, data=format_data(
        data), total_bits=TOTAL_BITS,
        frac_bits=FRAC_BITS, script=os.path.relpath(__file__))

    # Construct the absolute file path.
    here = os.path.dirname(os.path.realpath(__file__))
    output_file = os.path.realpath(os.path.join(here, OUTPUT_FILE))

    # Make sure the containing directory exists.
    os.makedirs(os.path.dirname(output_file), mode=0o775, exist_ok=True)

    # Write the file.
    with open(output_file, "w") as f:
        f.write(output)

    # Format the file according to style guide.
    subprocess.call(["rustfmt", output_file])

    # Show success message.
    print(f"Created file {output_file}")
