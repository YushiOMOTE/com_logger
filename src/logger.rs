use crate::serial::Serial;
use core::{fmt::write, format_args};
use log::*;
use spin::Mutex;

const COM1_PORT: u16 = 0x3f8;

static LOGGER: Logger = Logger(Mutex::new(COM1_PORT));

struct Logger(Mutex<u16>);

impl log::Log for Logger {
    fn enabled(&self, _m: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let port = self.0.lock();
        let mut serial = Serial::new(*port);

        let _ = write(
            &mut serial,
            format_args!(
                "{:>8}: {} ({}, {}:{})\n",
                record.level(),
                record.args(),
                record.target(),
                record.file().unwrap_or("<unknown>"),
                record.line().unwrap_or(0),
            ),
        );
    }

    fn flush(&self) {}
}

fn set_logger_base(base: u16) {
    let mut data = LOGGER.0.lock();
    *data = base;
}

/// The builder for a serial port logger.
///
/// The builder instance can be created also by [`builder`][] function.
pub struct Builder {
    base: u16,
    filter: LevelFilter,
}

impl Builder {
    /// Create a builder for fine-tuning logger.
    pub fn new() -> Self {
        Self {
            base: COM1_PORT,
            filter: LevelFilter::Info,
        }
    }

    /// Set the base address of a COM port.
    pub fn base(mut self, base: u16) -> Self {
        self.base = base;
        self
    }

    /// Set the level filter.
    pub fn filter(mut self, filter: LevelFilter) -> Self {
        self.filter = filter;
        self
    }

    /// Setup a logger based on the configuration.
    pub fn setup(self) {
        // Initialize serial port
        Serial::new(self.base).init();

        // Update base address of logger
        set_logger_base(self.base);

        set_logger(&LOGGER).unwrap();
        set_max_level(self.filter);
    }
}

/// Create a builder for fine-tuning logger.
///
/// Call [`Builder::setup`][] to apply the configuration and actually setup the logger.
///
/// ```rust,no_run
/// use log::*;
///
/// fn main() {
///    com_logger::builder()
///        .base(0x2f8)                  // Use COM2 port
///        .filter(LevelFilter::Debug)   // Print debug log
///        .setup();
///
///    debug!("Hello");
/// }
/// ```
pub fn builder() -> Builder {
    Builder::new()
}

/// Setup a logger with the default settings.
///
/// The default settings is COM1 port with level filter `Info`.
pub fn init() {
    builder().filter(LevelFilter::Info).setup();
}

/// Setup a logger with a custom level filter.
///
/// This overwrites the level filter of the default settings.
pub fn init_with_filter(filter: LevelFilter) {
    builder().filter(filter).setup();
}
