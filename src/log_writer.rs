pub trait LogWriter: Send + Sync {
    fn write(&self, message: &str) -> std::io::Result<()>;
}
