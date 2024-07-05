use crate::serial::Serial;
use core::sync::atomic::{spin_loop_hint, AtomicUsize, Ordering};
use core::{
    fmt::write,
    format_args,
    sync::atomic::{AtomicU16, Ordering},
};
use log::*;

const COM1_PORT: u16 = 0x3f8;

static LOGGER: Logger = Logger(Serial::new(COM1_PORT));

struct Mutex<T> {
    lock: AtomicUsize,
    data: T,
}

impl<T> Mutex<T> {
    fn new(data: T) -> Self {
        Mutex {
            lock: AtomicUsize::new(0),
            data,
        }
    }

    fn lock(&self) -> &T {
        while self.lock.swap(1, Ordering::Acquire) != 0 {
            spin_loop_hint();
        }
        &self.data
    }

    fn unlock(&self) {
        self.lock.store(0, Ordering::Release);
    }
}

struct Logger(Mutex<Serial>);

impl log::Log for Logger {
    fn enabled(&self, _m: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let serial = self.0.lock();

        let _ = write(
            &mut serial,
            format_args!(
                "[{}  {}] {}\n\0",
                record.level(),
                record.module_path().unwrap_or(record.target()),
                record.args()
            ),
        );
    }

    fn flush(&self) {}
}

fn set_logger_base(base: u16) {
    LOGGER.0.store(base, Ordering::Relaxed);
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
