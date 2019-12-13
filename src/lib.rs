//!
//! `com_logger` is a logger through COM port.
//!
//! This library is `no_std`, and doesn't rely on `alloc`.
//!
//! ```rust,no_run
//! use log::*;
//!
//! fn main() {
//!    com_logger::init();
//!
//!    info!("Hello");
//! }
//! ```
//!
//! The serial port base address and logging level filter can be configured.
//!
//! ```rust,no_run
//! use log::*;
//!
//! fn main() {
//!    com_logger::builder()
//!        .base(0x2f8)                  // Use COM2 port
//!        .filter(LevelFilter::Debug)   // Print debug log
//!        .setup();
//!
//!    debug!("Hello");
//! }
//! ```

#![no_std]
#![cfg_attr(feature = "readme", feature(external_doc))]
#![warn(missing_docs)]

#[cfg_attr(feature = "readme", doc(include = "../README.md"))]
type _Readme = ();

/// The module for the logger.
mod logger;

/// The module for the serial port driver.
mod serial;

pub use crate::logger::{builder, init, init_with_filter, Builder};
pub use crate::serial::Serial;
