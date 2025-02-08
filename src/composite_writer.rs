use crate::log_writer::LogWriter;
use std::io;

pub struct CompositeWriter {
    writers: Vec<Box<dyn LogWriter + Send + Sync>>,
}

impl CompositeWriter {
    /// Create a new empty CompositeWriter.
    pub fn new() -> Self {
        Self {
            writers: Vec::new(),
        }
    }

    /// Add a new writer to the composite.
    pub fn add_writer<W: LogWriter + Send + Sync + 'static>(&mut self, writer: W) {
        self.writers.push(Box::new(writer));
    }
}

impl LogWriter for CompositeWriter {
    fn write(&self, message: &str) -> io::Result<()> {
        // Loop through each writer and forward the message.
        for writer in &self.writers {
            writer.write(message)?;
        }
        Ok(())
    }
}
