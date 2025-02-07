pub Trait LogWriter: Send + Sync {
    fn write(&self, message: &str) -> std::io::Result<()>;
}
