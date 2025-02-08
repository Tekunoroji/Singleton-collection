use crate::st_logger::log_writer::LogWriter;
use colored::*;

pub struct ConsoleLogWriter;

impl LogWriter for ConsoleLogWriter {
    fn write(&self, message: &str) -> std::io::Result<()> {
        // Find the end of the timestamp (first closing bracket)
        let timestamp_end = match message.find("]") {
            Some(pos) => pos + 1,
            None => {
                println!("{}", message);
                return Ok(());
            }
        };
        let timestamp = &message[..timestamp_end];

        // Trim the rest and then find the next closing bracket for the level
        let rest = message[timestamp_end..].trim_start();
        let level_end = match rest.find("]") {
            Some(pos) => pos + 1,
            None => {
                println!("{}", message);
                return Ok(());
            }
        };
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
