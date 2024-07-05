use core::fmt;
use spin::Mutex;
use uart_16550::SerialPort;

/// Serial port driver which implements [`core::fmt::Write`][].
///
/// ```rust,no_run
/// use com_logger::Serial;
///
/// fn main() {
///    // Setup COM1 serial port.
///    let mut s = Serial::new(0x3f8);
///    s.init();
///
///    // Write the single byte to the serial port.
///    s.write(b'P');
///
///    // Write the string to the serial port.
///    core::fmt::write(&mut s, format_args!("Hello {}", 0xdead));
/// }
/// ```
pub struct Serial {
    port: Mutex<SerialPort>,
}

impl Serial {
    /// Create the driver instance for the specified base address.
    pub fn new(base: u16) -> Self {
        let port = unsafe { SerialPort::new(base) };

        Self {
            port: Mutex::new(port),
        }
    }

    /// Initialize the serial port.
    pub fn init(&mut self) {
        self.port.lock().init();
    }

    /// Write a single byte to the serial port.
    pub fn write(&mut self, d: u8) {
        self.port.lock().send(d);
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut port = self.port.lock();

        for b in s.bytes() {
            port.send(b);
        }
        Ok(())
    }
}
