use crate::composite_writer::CompositeWriter;
use crate::console_writer::ConsoleLogWriter;
use crate::file_writer::FileLogWriter;
use crate::log_writer::LogWriter;
use chrono::Local;
use once_cell::sync::Lazy;

use std::sync::Mutex;

#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub struct Logger<W: LogWriter> {
    writer: W,
}

impl<W: LogWriter> Logger<W> {
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    pub fn log(&self, level: LogLevel, message: &str) -> std::io::Result<()> {
        let timestamp = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();

        let raw_level = match level {
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Info => "[INFO]",
            LogLevel::Warning => "[WARNING]",
            LogLevel::Error => "[ERROR]",
        };

        let log_message = format!("{} {} {}", timestamp, raw_level, message);
        self.writer.write(&log_message)?; // Injected writer handles the logging
        Ok(())
    }
}

#[cfg(not(test))]
pub static LOGGER: Lazy<Mutex<Logger<CompositeWriter>>> = Lazy::new(|| {
    let mut composite = CompositeWriter::new();
    composite.add_writer(ConsoleLogWriter);
    composite.add_writer(FileLogWriter::new("app.log"));
    Mutex::new(Logger::new(composite))
});

#[cfg(test)]
pub static LOGGER: Lazy<Mutex<Logger<CompositeWriter>>> = Lazy::new(|| {
    let mut composite = CompositeWriter::new();
    composite.add_writer(ConsoleLogWriter);
    composite.add_writer(FileLogWriter::new("test_app.log"));
    Mutex::new(Logger::new(composite))
});

pub fn get_logger() -> &'static Mutex<Logger<CompositeWriter>> {
    &LOGGER
}

#[cfg(test)]
mod tests {
    use crate::globaldeb;
    use crate::globalerror;
    use crate::globalinfo;
    use crate::globalwarn;
    use regex::Regex;
    use std::fs::{remove_file, File};
    use std::io::Read;

    const TEST_LOG_FILE: &str = "test_app.log";

    fn read_file() -> String {
        let mut file = File::open(TEST_LOG_FILE).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    fn extract_first_timestamp(log_contents: &str) -> Option<String> {
        log_contents
            .lines()
            .next()?
            .split(['[', ']'].as_ref())
            .nth(1)
            .map(|s| s.trim().to_string())
    }

    fn reset_log_file() {
        std::fs::write(TEST_LOG_FILE, "").unwrap();
    }

    #[test]
    fn it_logs_messages_to_file() {
        reset_log_file();
        // Instead of manually locking and calling log, use the globalinfo! macro.
        globalinfo!("This is a log message");

        let log_contents = read_file();
        assert!(log_contents.contains("[INFO] This is a log message"));
    }

    #[test]
    fn it_logs_different_levels_correctly() {
        reset_log_file();
        globalinfo!("This is an info message");
        globalerror!("This is an error message");
        globalwarn!("This is a warning message");
        globaldeb!("This is a debug message");

        let log_contents = read_file();
        assert!(log_contents.contains("[INFO] This is an info message"));
        assert!(log_contents.contains("[ERROR] This is an error message"));
        assert!(log_contents.contains("[WARNING] This is a warning message"));
        assert!(log_contents.contains("[DEBUG] This is a debug message"));
    }

    #[test]
    fn it_appends_messages_to_file() {
        reset_log_file();
        globalinfo!("First log message");
        globalinfo!("Second log message");

        let log_contents = read_file();
        assert!(log_contents.contains("[INFO] First log message"));
        assert!(log_contents.contains("[INFO] Second log message"));
    }

    #[test]
    fn it_creates_file_if_not_exists() {
        remove_file(TEST_LOG_FILE).ok();
        globalinfo!("This is a log message");

        let log_contents = read_file();
        assert!(log_contents.contains("[INFO] This is a log message"));
    }

    #[test]
    fn it_logs_with_correct_timestamp_format() {
        reset_log_file();
        globalinfo!("Testing timestamp");

        let log_contents = read_file();
        let timestamp = extract_first_timestamp(&log_contents).expect("No timestamp found");

        let timestamp_pattern = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$").unwrap();
        assert!(
            timestamp_pattern.is_match(&timestamp),
            "Timestamp format is incorrect: {}",
            timestamp
        );
    }
}
