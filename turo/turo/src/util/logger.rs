use log::{
    Record, 
    Level, 
    Metadata,
    SetLoggerError,
    LevelFilter,
};

use crate::{serial_println, serial_print};


/* 
 * This logging implementation is based off of the k4dOS logging implementation, seen here:
 * https://github.com/clstatham/k4dos/blob/master/src/logging.rs, written by clstatham
 * on GitHub.
 */

struct TuroLogger;

impl log::Log for TuroLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
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

    fn flush(&self) {}
}

static LOGGER: TuroLogger = TuroLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(LevelFilter::Trace);
    Ok(())
}