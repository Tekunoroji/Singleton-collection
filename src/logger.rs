use chrono::Local;
use colored::*;
use once_cell::sync::Lazy;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

pub struct Logger {
    pub file_path: String,
}

impl Logger {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }

    pub fn log(&mut self, level: LogLevel, message: &str) -> std::io::Result<()> {
        let local = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();

        let raw_level = match level {
            LogLevel::Debug => "[DEBUG]",
            LogLevel::Info => "[INFO]",
            LogLevel::Warning => "[WARNING]",
            LogLevel::Error => "[ERROR]",
        };

        let colored_level = match level {
            LogLevel::Debug => "[DEBUG]".blue(),
            LogLevel::Info => "[INFO]".green(),
            LogLevel::Warning => "[WARNING]".yellow(),
            LogLevel::Error => "[ERROR]".red(),
        };

        let log_message = format!("{} {} {}", local, colored_level, message);
        println!("{}", log_message);

        let log_message = format!("{} {} {}\n", local, raw_level, message);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        file.write_all(log_message.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::new("test_app.log")));

#[cfg(not(test))]
pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::new("app.log")));

pub fn get_logger() -> &'static Mutex<Logger> {
    &LOGGER
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let logger = get_logger();
        {
            let mut log_instance = logger.lock().unwrap();
            match log_instance.log(LogLevel::Info, "This is a log message") {
                Ok(_) => println!("Log message written successfully"),
                Err(e) => println!("Error writing log message: {}", e),
            }
        }

        let log_contents = read_file();
        assert!(log_contents.contains("[INFO] This is a log message"));
    }

    #[test]
    fn it_logs_different_levels_correctly() {
        reset_log_file();
        let logger = get_logger();
        {
            let mut log_instance = logger.lock().unwrap();
            match log_instance.log(LogLevel::Info, "This is an info message") {
                Ok(_) => println!("Log message written successfully"),
                Err(e) => println!("Error writing log message: {}", e),
            }
            match log_instance.log(LogLevel::Error, "This is an error message") {
                Ok(_) => println!("Log message written successfully"),
                Err(e) => println!("Error writing log message: {}", e),
            }
            match log_instance.log(LogLevel::Warning, "This is a warning message") {
                Ok(_) => println!("Log message written successfully"),
                Err(e) => println!("Error writing log message: {}", e),
            }
        }

        let log_contents = read_file();
        assert!(log_contents.contains("[INFO] This is an info message"));
        assert!(log_contents.contains("[ERROR] This is an error message"));
        assert!(log_contents.contains("[WARNING] This is a warning message"));
    }

    #[test]
    fn it_appends_messages_to_file() {
        reset_log_file();
        let logger = get_logger();
        {
            let mut log_instance = logger.lock().unwrap();
            match log_instance.log(LogLevel::Info, "First log message") {
                Ok(_) => println!("Log message written successfully"),
                Err(e) => println!("Error writing log message: {}", e),
            }
            match log_instance.log(LogLevel::Info, "Second log message") {
                Ok(_) => println!("Log message written successfully"),
                Err(e) => println!("Error writing log message: {}", e),
            }
        }

        let log_contents = read_file();
        assert!(log_contents.contains("[INFO] First log message"));
        assert!(log_contents.contains("[INFO] Second log message"));
    }

    #[test]

    fn it_creates_file_if_not_exists() {
        remove_file(TEST_LOG_FILE).ok();
        let logger = get_logger();
        {
            let mut log_instance = logger.lock().unwrap();
            match log_instance.log(LogLevel::Info, "This is a log message") {
                Ok(_) => println!("Log message written successfully"),
                Err(e) => println!("Error writing log message: {}", e),
            }
        }
        let log_contents = read_file();
        assert!(log_contents.contains("[INFO] This is a log message"));
    }

    #[test]
    fn it_logs_with_correct_timestamp_format() {
        reset_log_file();
        let logger = get_logger();
        {
            let mut log_instance = logger.lock().unwrap();
            match log_instance.log(LogLevel::Info, "Testing timestamp") {
                Ok(_) => println!("Log message written successfully"),
                Err(e) => println!("Error writing log message: {}", e),
            };
        }

        let log_contents = read_file();
        let timestamp = extract_first_timestamp(&log_contents).expect("No timestamp found");

        let timestamp_pattern =
            regex::Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$").unwrap();
        assert!(
            timestamp_pattern.is_match(&timestamp),
            "Timestamp format is incorrect: {}",
            timestamp
        );
    }
}
