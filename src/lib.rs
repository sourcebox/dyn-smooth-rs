#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

mod tan;

/// Number of fractional bits for the i32 implementation.
pub const I32_FRAC_BITS: u8 = tan::TAN_LUT_FRAC_BITS;

////////////////////////////////////////////////////////////////////////////////

/// Efficient variant of the smoother using f32.
#[derive(Debug)]
pub struct DynamicSmootherEcoF32 {
    /// Lowpass 1 signal.
    low1: f32,

    /// Lowpass 2 signal.
    low2: f32,

    /// Base filter coefficient calculated from `basefreq`.
    g0: f32,

    /// Sensitivity of the dynamic response.
    sense: f32,
}

/// Implementation of the efficient smoother using single-precision floats.
impl DynamicSmootherEcoF32 {
    /// Returns a new smoother.
    /// - `basefreq` is the initial cutoff frequency.
    /// - `samplerate` is the sampling frequency.
    /// - `sensitivity` sets the amount of dynamic cutoff response.
    pub fn new(basefreq: f32, samplerate: f32, sensitivity: f32) -> Self {
        // Normalized frequency in cycles per sample.
        // Must be in range 0..0.5
        let wc = basefreq / samplerate;

        // Base filter coefficient.
        let gc = tan_wc_f32(wc);
        let g0 = 2.0 * gc / (1.0 + gc);

        Self {
            low1: 0.0,
            low2: 0.0,
            g0,
            sense: sensitivity * 4.0,
        }
    }

    /// Resets all internal values.
    pub fn clear(&mut self) {
        self.low1 = 0.0;
        self.low2 = 0.0;
    }

    /// Processes a new input value and returns the smoothed output.
    pub fn tick(&mut self, input: f32) -> f32 {
        // Store values from previous sample.
        let low1z = self.low1;
        let low2z = self.low2;

        // Get bandpass response as difference of the 2 lowpasses.
        // Identical to an SVF bandpass output dampened by 2.
        let bandz = abs_f32(low1z - low2z);

        // Get the filter coefficient from the base value and a dynamic
        // amount depending on the bandpass response and sensitivity.
        // Value is clamped to 1.
        let g = f32::min(self.g0 + self.sense * bandz, 1.0);

        // Calculate lowpass 1 signal based on input.
        self.low1 = low1z + g * (input - low1z);

        // Calculate lowpass 2 signal based on lowpass 1 output.
        self.low2 = low2z + g * (self.low1 - low2z);

        // Return lowpass 2 signal as output.
        self.low2
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Efficient variant of the smoother using i32.
#[derive(Debug)]
pub struct DynamicSmootherEcoI32 {
    /// Lowpass 1 signal.
    low1: i32,

    /// Lowpass 2 signal.
    low2: i32,

    /// Base filter coefficient calculated from `basefreq`.
    g0: i32,

    /// Sensitivity of the dynamic response.
    sense: i32,
}

/// Implementation of the efficient smoother using 32-bit integers.   
/// Important: Because of internal fractional processing, the value range is limited to about 23-bit.
impl DynamicSmootherEcoI32 {
    /// Returns a new smoother.
    /// - `basefreq` is the initial cutoff frequency.
    /// - `samplerate` is the sampling frequency.
    /// - `sensitivity` sets the amount of dynamic cutoff response.
    ///
    /// All values must have a fractional part of `I32_FRAC_BITS`.
    pub fn new(basefreq: i32, samplerate: i32, sensitivity: i32) -> Self {
        // Normalized frequency in cycles per sample.
        // Must be in range 0..(1 << LUT_FRAC_BITS)/2
        let wc = ((basefreq as i64) * (1 << tan::TAN_LUT_FRAC_BITS) / (samplerate as i64)) as i32;

        // Base filter coefficient.
        let gc = tan_wc_i32(wc);
        let g0 = ((2 * gc) << tan::TAN_LUT_FRAC_BITS) / ((1 << tan::TAN_LUT_FRAC_BITS) + gc);

        Self {
            low1: 0,
            low2: 0,
            g0,
            sense: sensitivity * 4,
        }
    }

    /// Resets all internal values.
    pub fn clear(&mut self) {
        self.low1 = 0;
        self.low2 = 0;
    }

    /// Processes a new input value and returns the smoothed output.
    pub fn tick(&mut self, input: i32) -> i32 {
        /// Improve calculation by using fractional bits internally.
        const INTERNAL_FRAC_BITS: u8 = 8;

        let input = input << INTERNAL_FRAC_BITS;

        // Store values from previous sample.
        let low1z = self.low1;
        let low2z = self.low2;

        // Get bandpass response as difference of the 2 lowpasses.
        // Identical to an SVF bandpass output dampened by 2.
        let bandz = (low1z - low2z).abs();

        // Get the filter coefficient from the base value and a dynamic
        // amount depending on the bandpass response and sensitivity.
        // Value is clamped to 1.
        let g = i32::min(
            self.g0 + ((self.sense as i64 * bandz as i64) >> INTERNAL_FRAC_BITS) as i32,
            1 << tan::TAN_LUT_FRAC_BITS,
        );

        // Calculate lowpass 1 signal based on input.
        self.low1 = (low1z as i64
            + (((g as i64) * (input - low1z) as i64) >> tan::TAN_LUT_FRAC_BITS))
            as i32;

        // Calculate lowpass 2 signal based on lowpass 1 output.
        self.low2 = (low2z as i64
            + (((g as i64) * (self.low1 - low2z) as i64) >> tan::TAN_LUT_FRAC_BITS))
            as i32;

        // Return lowpass 2 signal as output.
        self.low2 >> INTERNAL_FRAC_BITS
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Return tangent value for normalized frequency `wc` in range 0..0.5
fn tan_wc_f32(wc: f32) -> f32 {
    use tan::*;

    let lut_index = (wc * 2.0 * TAN_LUT_LENGTH as f32 * (1 << TAN_LUT_FRAC_BITS) as f32) as usize;
    let lut_index_int = lut_index >> TAN_LUT_FRAC_BITS;
    let lut_index_frac = lut_index & ((1 << TAN_LUT_FRAC_BITS) - 1);

    let lut_index1 = lut_index_int.min(TAN_LUT_LENGTH - 2);
    let lut_index2 = lut_index1 + 1;

    let result = (TAN_LUT[lut_index1] as i64 * ((1 << TAN_LUT_FRAC_BITS) - lut_index_frac as i64)
        + TAN_LUT[lut_index2] as i64 * (lut_index_frac as i64))
        >> TAN_LUT_FRAC_BITS;

    (result as f32) / (1 << TAN_LUT_FRAC_BITS) as f32
}

/// Return tangent value for normalized frequency `wc` in range 0..(1 << LUT_FRAC_BITS)/2
fn tan_wc_i32(wc: i32) -> i32 {
    use tan::*;

    let lut_index = (wc * 2) as usize * TAN_LUT_LENGTH;
    let lut_index_int = lut_index >> TAN_LUT_FRAC_BITS;
    let lut_index_frac = lut_index & ((1 << TAN_LUT_FRAC_BITS) - 1);

    let lut_index1 = lut_index_int.min(TAN_LUT_LENGTH - 2);
    let lut_index2 = lut_index1 + 1;

    (((TAN_LUT[lut_index1] as i64 * ((1 << TAN_LUT_FRAC_BITS) - lut_index_frac as i64))
        >> TAN_LUT_FRAC_BITS)
        + ((TAN_LUT[lut_index2] as i64 * (lut_index_frac as i64)) >> TAN_LUT_FRAC_BITS)) as i32
}

/// Return abs(x) for f32 values.
///
/// This custom helper function is required because `f32::abs()`
/// does not exist in `no_std` environments.
fn abs_f32(x: f32) -> f32 {
    if x < 0.0 {
        -x
    } else {
        x
    }
}

#[cfg(test)]
mod tests;
