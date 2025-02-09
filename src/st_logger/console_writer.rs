use crate::st_logger::log_writer::LogWriter;
use colored::*;
use std::io::{Error, ErrorKind};

pub struct ConsoleLogWriter;

impl LogWriter for ConsoleLogWriter {
    fn write(&self, message: &str) -> std::io::Result<()> {
        let timestamp_end = message.find("]").ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Missing closing bracket for timestamp in message: {}",
                    message
                ),
            )
        })? + 1;
        let timestamp = &message[..timestamp_end];

        let rest = message[timestamp_end..].trim_start();
        let level_end = rest.find("]").ok_or_else(|| {
            Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Missing closing bracket for log level in message: {}",
                    message
                ),
            )
        })? + 1;
        let level = &rest[..level_end];
        let content = rest[level_end..].trim_start();

        let colored_level = match level {
            "[DEBUG]" => "[DEBUG]".blue(),
            "[INFO]" => "[INFO]".green(),
            "[WARNING]" => "[WARNING]".yellow(),
            "[ERROR]" => "[ERROR]".red(),
            _ => "[UNKNOWN]".normal(),
        };

        println!("{} {} {}", timestamp, colored_level, content);
        Ok(())
    }
}
