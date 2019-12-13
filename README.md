# com_logger

Serial port logger through COM ports.

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
