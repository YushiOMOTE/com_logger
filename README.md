# com_logger

[![Latest version](https://img.shields.io/crates/v/com_logger.svg)](https://crates.io/crates/com_logger)
[![Documentation](https://docs.rs/com_logger/badge.svg)](https://docs.rs/com_logger)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Actions Status](https://github.com/YushiOMOTE/com_logger/workflows/Rust/badge.svg)](https://github.com/YushiOMOTE/com_logger/actions)

Serial port logger through [COM ports](https://en.wikipedia.org/wiki/COM_(hardware_interface)).

* Doesn't require `std` (`no_std`)
* Doesn't require `alloc`

```rust,no_run
use log::*;

fn main() {
    com_logger::init();

    info!("Starting");
}
```

## Configuration

```rust,no_run
use log::*;

fn main() {
    com_logger::builder()
        .base(0x2f8)                  // Use COM2 port
        .filter(LevelFilter::Debug)   // Print debug log
        .setup();

    debug!("Hello");
}
```
