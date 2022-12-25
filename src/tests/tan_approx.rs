//! Test for the tangent approximation via lookup table.

use crate::*;

#[test]
/// Check deviation for f32 LUT-based tangent function by comparing
/// the results with those from the standard library.
fn tan_lut_deviation_f32() {
    const DEVIATION_MAX_PERCENT: f32 = 5.0;

    println!();
    println!("|  deg  |  rad  |  wc   | std tan | lut tan |   dev    |");
    println!("|-------|-------|-------|---------|---------|----------|");

    for degrees in 0..90 {
        let radiant = degrees as f32 * std::f32::consts::PI / 180.0;

        // Value from standard library for reference
        let tan_std = radiant.tan();

        // Value from lookup table
        let wc = radiant / std::f32::consts::PI;
        let tan_lut = tan_wc_f32(wc);

        // Absolute and relative deviation
        let deviation = tan_lut - tan_std;
        let deviation_percent = if tan_std > 0.0 {
            deviation * 100.0 / tan_std
        } else {
            0.0
        };

        println!(
            "| {:>4}° | {:>5.3} | {:>5.3} | {:>7.3} | {:>7.3} | {:>7.3}% |",
            degrees, radiant, wc, tan_std, tan_lut, deviation_percent
        );

        // Ignore deviation for values close to 90*.
        // Otherwise LUT has to be much larger for better precision.
        let ignore = degrees >= 85;

        assert!(
            deviation_percent.abs() <= DEVIATION_MAX_PERCENT || ignore,
            "Deviation is {:.3}% for {}°",
            deviation_percent,
            degrees
        );
    }

    println!();
}

#[test]
/// Check deviation for i32 LUT-based tangent function by comparing
/// the results with those from the standard library.
fn tan_lut_deviation_i32() {
    const DEVIATION_MAX_PERCENT: f32 = 5.0;

    println!();
    println!("|  deg  |  rad  |  wc   | std tan | lut tan |   dev    |");
    println!("|-------|-------|-------|---------|---------|----------|");

    for degrees in 0..90 {
        let radiant = degrees as f32 * std::f32::consts::PI / 180.0;

        // Value from standard library for reference
        let tan_std = radiant.tan();

        // Value from lookup table
        const SCALE_FACTOR: f32 = (1 << tan::TAN_LUT_FRAC_BITS) as f32;
        let wc = (radiant * SCALE_FACTOR / std::f32::consts::PI) as i32;
        let tan_lut = tan_wc_i32(wc) as f32 / SCALE_FACTOR;

        // Absolute and relative deviation
        let deviation = tan_lut - tan_std;
        let deviation_percent = if tan_std > 0.0 {
            deviation * 100.0 / tan_std
        } else {
            0.0
        };

        println!(
            "| {:>4}° | {:>5.3} | {:>5.3} | {:>7.3} | {:>7.3} | {:>7.3}% |",
            degrees, radiant, wc, tan_std, tan_lut, deviation_percent
        );

        // Ignore deviation for values close to 90*.
        // Otherwise LUT has to be much larger for better precision.
        let ignore = degrees >= 85;

        assert!(
            deviation_percent.abs() <= DEVIATION_MAX_PERCENT || ignore,
            "Deviation is {:.3}% for {}°",
            deviation_percent,
            degrees
        );
    }

    println!();
}
