# lis2hh12-rs

A platform-agnostic, `no_std` driver for the STMicroelectronics LIS2HH12 3-axis accelerometer, built on [`embedded-hal`](https://crates.io/crates/embedded-hal) 1.0 and the [`accelerometer`](https://crates.io/crates/accelerometer) crate.

## Requirements

- Rust edition 2021, MSRV **1.75**
- `embedded-hal` **1.0**
- `accelerometer` **0.12**

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
lis2hh12-rs = "1.0"
```

Create a driver instance with any `embedded_hal::i2c::I2c` implementation:

```rust
use lis2hh12_rs::{Lis2hh12, DataRate};
use accelerometer::RawAccelerometer;

let mut accel = Lis2hh12::new(i2c, 0x1E).unwrap();

// Change the data rate
accel.set_datarate(DataRate::Hz_100).unwrap();

// Read raw acceleration
let raw = accel.accel_raw().unwrap();
```

## Acknowledgments

Based on [BenBergman's lis3dh driver](https://github.com/BenBergman/lis3dh-rs).

## License

MIT
