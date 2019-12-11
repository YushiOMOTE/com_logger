use x86_64::instructions::port::Port;

pub struct Serial {
    ports: [Port<u8>; 8],
}

impl Serial {
    pub fn new() -> Self {
        let mut ports = [
            Port::new(0x3f8),
            Port::new(0x3f9),
            Port::new(0x3fa),
            Port::new(0x3fb),
            Port::new(0x3fc),
            Port::new(0x3fd),
            Port::new(0x3fe),
            Port::new(0x3ff),
        ];

        Self { ports }
    }

    pub fn init(&mut self) {
        unsafe {
            self.ports[1].write(0x00);
            self.ports[3].write(0x80);
            self.ports[0].write(0x03);
            self.ports[1].write(0x00);
            self.ports[3].write(0x03);
            self.ports[2].write(0xc7);
            self.ports[4].write(0x0b);
        }
    }

    pub fn read(&mut self) -> Option<u8> {
        unsafe {
            if self.ports[5].read() & 0x01 != 0 {
                Some(self.ports[0].read())
            } else {
                None
            }
        }
    }

    pub fn write(&mut self, d: u8) {
        unsafe {
            while self.ports[5].read() & 0x20 == 0 {}
            self.ports[0].write(d);
        }
    }
}
