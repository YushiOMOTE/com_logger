use crate::serial::Serial;
use core::{fmt::Write, writeln};
use log::*;
use spin::Mutex;

const COM1_PORT: u16 = 0x3f8;

static LOGGER: Logger = Logger::new(COM1_PORT);

/// Formatter that converts log records into desirable text format.
pub type Formatter = fn(&mut dyn Write, &Record) -> Result<(), core::fmt::Error>;

struct Inner {
    base: u16,
    formatter: Formatter,
}

struct Logger {
    inner: Mutex<Inner>,
}

impl Logger {
    const fn new(base: u16) -> Self {
        Logger {
            inner: Mutex::new(Inner {
                base,
                formatter: default_formatter,
            }),
        }
    }

    fn set_base(&self, new_base: u16) {
        let mut inner = self.inner.lock();
        inner.base = new_base;
    }

    fn set_formatter(&self, new_formatter: Formatter) {
        let mut inner = self.inner.lock();
        inner.formatter = new_formatter;
    }
}

impl log::Log for Logger {
    fn enabled(&self, _m: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let inner = self.inner.lock();
        let mut serial = Serial::new(inner.base);

        let _ = (inner.formatter)(&mut serial, record);
    }

    fn flush(&self) {}
}

fn default_formatter(buffer: &mut dyn Write, record: &Record) -> Result<(), core::fmt::Error> {
    writeln!(
        buffer,
        "{:>8}: {} ({}, {}:{})\n",
        record.level(),
        record.args(),
        record.target(),
        record.file().unwrap_or("<unknown>"),
        record.line().unwrap_or(0)
    )
}

/// The builder for a serial port logger.
///
/// The builder instance can be created also by [`builder`][] function.
pub struct Builder {
    base: u16,
    filter: LevelFilter,
    formatter: Formatter,
}

impl Builder {
    /// Create a builder for fine-tuning logger.
    pub fn new() -> Self {
        Self {
            base: COM1_PORT,
            filter: LevelFilter::Info,
            formatter: default_formatter,
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

    /// Set formatter.
    pub fn formatter(mut self, formatter: Formatter) -> Self {
        self.formatter = formatter;
        self
    }

    /// Setup a logger based on the configuration.
    pub fn setup(self) {
        // Initialize serial port
        Serial::new(self.base).init();

        // Update base address of logger
        LOGGER.set_base(self.base);
        LOGGER.set_formatter(self.formatter);

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
///        .formatter(|buf, record| writeln!(buf, "{}", record.args())) // Define own format
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
