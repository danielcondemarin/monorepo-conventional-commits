use log::{Level, Metadata, Record, SetLoggerError};
use std::io::Write;
use std::{
    env,
    fs::{File, OpenOptions},
    sync::Mutex,
};

pub struct Logger {
    writer: Option<Mutex<Box<File>>>,
}

static DEBUG_LOG_FILE: &str = "NVIM_DEBUG_LOG";

impl Logger {
    pub fn new() -> Logger {
        if let Ok(log_file) = env::var(DEBUG_LOG_FILE) {
            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(log_file)
                .expect("failed to open logging file");

            Logger {
                writer: Some(Mutex::new(Box::new(file))),
            }
        } else {
            Logger { writer: None }
        }
    }

    pub fn init(self) -> Result<(), SetLoggerError> {
        log::set_boxed_logger(Box::new(self)).map(|()| log::set_max_level(log::LevelFilter::Info))
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        env::var(DEBUG_LOG_FILE).is_ok() && metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("{}-{} {}", record.level(), record.target(), record.args());
            if let Some(writer) = &self.writer {
                writer
                    .lock()
                    .unwrap()
                    .write_all(msg.as_bytes())
                    .expect("failed to write to log file");
            }
        }
    }

    fn flush(&self) {
        if let Some(writer) = &self.writer {
            writer
                .lock()
                .unwrap()
                .flush()
                .expect("failed to flush logger");
        }
    }
}
