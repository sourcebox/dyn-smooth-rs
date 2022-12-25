//! Lookup table for fixed-point tangent function.
//!
//! This file is auto-generated. Don't edit!
//! Build script: tools/lut-tan-i32.py

/// Number of table entries.
pub(crate) const TAN_LUT_LENGTH: usize = 32;

/// Number of fractional bits used in table values.
pub(crate) const TAN_LUT_FRAC_BITS: u8 = 16;

/// Table data in the range 0..PI/2.
pub(crate) const TAN_LUT: [i32; 32] = [
    0, 3219, 6454, 9721, 13035, 16415, 19880, 23449, 27145, 30996, 35029, 39280, 43789, 48604,
    53784, 59398, 65535, 72307, 79855, 88365, 98081, 109340, 122609, 138564, 158217, 183160,
    216043, 261634, 329471, 441807, 665398, 1334015,
];
