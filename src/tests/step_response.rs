//! Test for step response.

use crate::{DynamicSmootherEcoF32, DynamicSmootherEcoI32, I32_FRAC_BITS};

#[test]
/// Check step response with parameters from original paper.
fn step_response_default() {
    const NUM_SAMPLES: usize = 20;
    const INPUT_VALUE: f32 = 1000.0;

    const BASEFREQ: f32 = 2.0;
    const SAMPLERATE: f32 = 1000.0;
    const SENSITIVITY: f32 = 0.5;

    let mut smoother_f32 = DynamicSmootherEcoF32::new(BASEFREQ, SAMPLERATE, SENSITIVITY);
    let mut smoother_i32 = DynamicSmootherEcoI32::new(
        (BASEFREQ * (1 << I32_FRAC_BITS) as f32) as i32,
        (SAMPLERATE * (1 << I32_FRAC_BITS) as f32) as i32,
        (SENSITIVITY * (1 << I32_FRAC_BITS) as f32) as i32,
    );

    let mut rows = Vec::new();

    for n in 0..NUM_SAMPLES {
        let input_value = if n < NUM_SAMPLES / 2 {
            INPUT_VALUE
        } else {
            INPUT_VALUE * 0.9
        };

        let output_f32 = smoother_f32.tick(input_value);
        let output_i32 = smoother_i32.tick(input_value as i32);

        rows.push((input_value, output_f32, output_i32));
    }

    print_table(&rows);
}

#[test]
/// Check step response with low cutoff frequency.
fn step_response_low_freq() {
    const NUM_SAMPLES: usize = 20;
    const INPUT_VALUE: f32 = 1000.0;

    const BASEFREQ: f32 = 0.05;
    const SAMPLERATE: f32 = 100.0;
    const SENSITIVITY: f32 = 0.05;

    let mut smoother_f32 = DynamicSmootherEcoF32::new(BASEFREQ, SAMPLERATE, SENSITIVITY);
    let mut smoother_i32 = DynamicSmootherEcoI32::new(
        (BASEFREQ * (1 << I32_FRAC_BITS) as f32) as i32,
        (SAMPLERATE * (1 << I32_FRAC_BITS) as f32) as i32,
        (SENSITIVITY * (1 << I32_FRAC_BITS) as f32) as i32,
    );

    let mut rows = Vec::new();

    for n in 0..NUM_SAMPLES {
        let input_value = if n < NUM_SAMPLES / 2 {
            INPUT_VALUE
        } else {
            INPUT_VALUE * 0.9
        };

        let output_f32 = smoother_f32.tick(input_value);
        let output_i32 = smoother_i32.tick(input_value as i32);

        rows.push((input_value, output_f32, output_i32));
    }

    print_table(&rows);
}
fn print_table(rows: &[(f32, f32, i32)]) {
    println!();
    println!("|  #  | input |   f32 out  |   i32 out  |");
    println!("|-----|-------|------------|------------|");

    for (n, row) in rows.iter().enumerate() {
        println!(
            "| {:>3} | {:>5} | {:>10.3} | {:>10.3} |",
            n, row.0, row.1, row.2,
        );
    }

    println!();
}
