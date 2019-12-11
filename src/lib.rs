#![no_std]

extern crate alloc;

pub mod serial;

use crate::serial::Serial;
use log::*;

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _m: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let msg = alloc::format!(
            "{:>8}: {} ({}, {}:{})\n",
            record.level(),
            record.args(),
            record.target(),
            record.file().unwrap_or("<unknown>"),
            record.line().unwrap_or(0),
        );

        let mut s = Serial::new();
        for b in msg.bytes() {
            s.write(b);
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    init_with_filter(LevelFilter::Debug);
}

pub fn init_with_filter(filter: LevelFilter) {
    Serial::new().init();
    set_logger(&Logger).unwrap();
    set_max_level(filter);
}
