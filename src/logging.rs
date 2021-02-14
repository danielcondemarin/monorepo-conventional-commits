use log::{Level, Metadata, Record, SetLoggerError};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    sync::Mutex,
};

pub struct Logger {
    writer: Mutex<Box<File>>,
}

impl Logger {
    pub fn new() -> Logger {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("/Users/daniel/workspace/nvim-conventional-commits/log.txt")
            .expect("failed to open logging file");

        let writer = Mutex::new(Box::new(file));

        Logger { writer }
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(log::LevelFilter::Info))
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("{}-{} {}", record.level(), record.target(), record.args());

            if let Err(error) = self.writer.lock().unwrap().write_all(msg.as_bytes()) {
                let crash_report = File::create(
                    "/Users/daniel/workspace/nvim-conventional-commits/crash_report.txt",
                );

                crash_report
                    .unwrap()
                    .write_all(format!("Unexpected error ocurred. {}", error).as_bytes())
                    .expect("failed to produce crash report");
            }
        }
    }

    fn flush(&self) {
        self.writer
            .lock()
            .unwrap()
            .flush()
            .expect("failed to flush logger");
    }
}
