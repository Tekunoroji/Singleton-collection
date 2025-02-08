use crate::log_writer::LogWriter;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

pub struct FileLogWriter {
    pub file_path: String,
    pub file_mutex: Mutex<()>,
}

impl FileLogWriter {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.into(),
            file_mutex: Mutex::new(()),
        }
    }
}

impl LogWriter for FileLogWriter {
    fn write(&self, message: &str) -> std::io::Result<()> {
        let _lock = self.file_mutex.lock().unwrap();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)?;

        writeln!(file, "{}", message)?;
        Ok(())
    }
}
