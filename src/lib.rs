#![no_std]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

/// The module for the logger.
mod logger;

/// The module for the serial port driver.
mod serial;

pub use crate::logger::{builder, init, init_with_filter, Builder};
pub use crate::serial::Serial;
