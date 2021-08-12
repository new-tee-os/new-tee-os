use hal::println;
use log::{LevelFilter, Log, SetLoggerError};

struct ConsoleLogger;

static LOGGER: ConsoleLogger = ConsoleLogger;

impl Log for ConsoleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        println!("[{}] {}", record.level(), record.args());
    }

    fn flush(&self) {}
}

pub fn klog_init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(LevelFilter::Debug);
    Ok(())
}
