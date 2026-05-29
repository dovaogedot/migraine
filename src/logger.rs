use log::Level;

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let clear = "\x1b[0m";

            let color = match record.level() {
                Level::Error => "\x1b[31m", // Red
                Level::Warn => "\x1b[33m",  // Yellow
                Level::Info => "\x1b[32m",  // Green
                Level::Debug => "\x1b[36m", // Cyan
                Level::Trace => "\x1b[35m", // Magenta
            };

            match record.level() {
                Level::Error => println!("{color}{}{clear}", record.args()),
                Level::Warn => println!("{color}{}{clear}", record.args()),
                Level::Info => println!("{}", record.args()),
                Level::Debug => println!("   {}", record.args()),
                Level::Trace => println!("     {}", record.args()),
            }
        }
    }

    fn flush(&self) {}
}
