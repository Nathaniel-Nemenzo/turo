use log::Level;
use log::Log;

use crate::serial_print;
use crate::serial_println;

struct TuroLogger;

impl Log for TuroLogger {
    // TODO: figure out what to do here
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        true
    }

    /// Log to the host using the serial0 interface.
    fn log(&self, record: &log::Record) {
        match record.level() {
            Level::Debug => serial_print!("\x1b[1;32m"),
            Level::Error => serial_print!("\x1b[1;31m"),
            Level::Info => serial_print!("\x1b[1;36m"),
            Level::Warn => serial_print!("\x1b[1;33m"),
            Level::Trace => serial_print!("\x1b[1;37m"),
        }

        serial_print!(
            "[{}]\t{}",
            // record.file().unwrap_or("(no file)"),
            // record.line().unwrap_or(0),
            record.level(),
            record.args()
        );
        serial_println!("\x1b[0m");
    }

    // TODO: flush logger
    fn flush(&self) {}
}

pub fn init() {
    log::set_logger(&TuroLogger).expect("Error setting logger");
    log::set_max_level(log::LevelFilter::Trace);
}