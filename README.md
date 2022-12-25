# dyn-smooth

This `no_std` Rust crate implements a dynamic smoothing filter based on an [algorithm by
Andrew Simper](https://cytomic.com/files/dsp/DynamicSmoothing.pdf).

It uses a lowpass filter whose cutoff frequency is modulated dynamically by the signal itself and allows to track signal changes faster than a purely static version.

The main use of this crate is to smooth noisy or stepped signals like analog measurements from an ADC in embedded systems.

## Status

Currently there are only implementations of the "efficient" variant using single-precision floats and 32-bit integers.

## Usage Examples

### Floating point

```rust
use dyn_smooth::DynamicSmootherEcoF32;

// Create an instance with suitable settings.
let base_freq = 2.0;
let sample_freq = 1000.0;
let sensitivity = 0.5;
let mut smoother = DynamicSmootherEcoF32::new(base_freq, sample_freq, sensitivity);

// Feed an input value to the smoother and retrieve the smoothed value.
for sample in 0..100 {
    let input_value = sample as f32;   // Dummy value, read some real value from somewhere
    let smoothed_value = smoother.tick(input_value);
}
```

### Integer

```rust
use dyn_smooth::{DynamicSmootherEcoI32, I32_FRAC_BITS};

// Create an instance with suitable settings.
let base_freq = 2 << I32_FRAC_BITS;
let sample_freq = 1000 << I32_FRAC_BITS;
let sensitivity = (0.5 * ((1 << I32_FRAC_BITS) as f32)) as i32;
let mut smoother = DynamicSmootherEcoI32::new(base_freq, sample_freq, sensitivity);

// Feed an input value to the smoother and retrieve the smoothed value.
for sample in 0..100 {
    let input_value = sample as i32;   // Dummy value, read some real value from somewhere
    let smoothed_value = smoother.tick(input_value);
}
```

## Tests

Run `cargo test` for the unit tests. Use the `--nocapture` option for additional output.

## License

Published under the MIT license. All contributions to this project must be provided under the same license conditions.

Author: Oliver Rockstedt <info@sourcebox.de>
