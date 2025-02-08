# 🦀 Rust Singleton Logger

A robust, thread-safe, and modular logger implemented in Rust. This logger leverages the power of a global singleton, dependency injection, and macros to provide a flexible logging solution that can be easily extended with new writers (such as JSON or remote loggers) without changing its core logic.

---

## 🚀 Features

- **Singleton Pattern**  
  Uses a global `LOGGER` (via `once_cell::sync::Lazy` and a `Mutex`) to ensure a single, unified logger is used throughout your application.

- **Dependency Injection & Modular Writers**  
  The logger is generic over a `LogWriter` trait. A composite writer aggregates multiple log writers (e.g., console and file), allowing you to easily add, remove, or replace log destinations.

- **Macro-based Logging**  
  Provides simple macros (e.g., `globaldeb!`, `globalinfo!`, `globalwarn!`, `globalerror!`) for convenient logging with different levels. This hides the boilerplate of locking and formatting, letting you log with one-line calls.

- **Colored Terminal Output**  
  Uses the [colored](https://docs.rs/colored) crate to color-code log messages in the terminal by log level.

- **File Logging**  
  Automatically creates and appends plain-text log entries to `app.log` (or `test_app.log` during tests).

- **Extensible Design**  
  Easily extend the logger by adding new writer implementations (for example, a JSON writer) without modifying the core logging logic.

- **Thread Safety**  
  All log calls are guarded by a `Mutex` ensuring that concurrent writes are safe and deterministic.

- **Comprehensive Testing**  
  A full test suite verifies log file creation, timestamp formatting, log level categorization, and message appending.

---

## 📝 Usage

### **Logging via Macros**

Once configured, you can log messages using simple macros:

```rust
globaldeb!("Debug message: {}", some_debug_value);
globalinfo!("Info message: {}", some_info);
globalwarn!("Warning: {}", some_warning);
globalerror!("Error occurred: {}", some_error);
```

Each macro accesses the global singleton logger, formats the message with a timestamp and log level, and dispatches it to all registered writers (e.g., console and file).

### **Initialization**

The global logger is configured in your crate’s source (see `global_logger.rs`):

- **Production**  
  Writes to `app.log` and outputs to the console.
  
- **Testing**  
  Writes to `test_app.log` and outputs to the console.

You only need to include the module in your main file:

```rust
#[macro_use]
mod macros;
mod global_logger;
mod logger;
mod composite_writer;
mod file_writer;
mod console_writer;

fn main() {
    globalinfo!("Application started!");
    // ... rest of your application
}
```

---

## ✅ Running Tests

Run all tests using Cargo:

```sh
cargo test
```

The tests verify that:
- Log files are created correctly.
- Timestamps are properly formatted.
- Logs are correctly appended and categorized by log level.
- The global singleton logger behaves as expected in both test and production configurations.

---

## 🔮 Roadmap & Extensions

- **Async Logging:**  
  Consider integrating with [Tokio](https://tokio.rs/) or using channels for non-blocking log writes.

- **Integration with `log` or `tracing`:**  
  Implement the `Log` trait or integrate with the [tracing](https://docs.rs/tracing) ecosystem for structured logging.

- **Log Rotation:**  
  Add features for size-based or time-based log rotation to manage log file growth in production environments.

- **Additional Writers:**  
  Easily extend the logger by adding new writers (e.g., JSON, remote logging) using the dependency injection pattern.

---

## 🌟 Conclusion

This logger combines industry best practices:
- A **singleton** global logger for unified access.
- **Dependency injection** for modularity and extensibility.
- **Macros** for simple and consistent logging calls.

It’s a flexible foundation that can grow with your application’s needs. Enjoy building with Rust and happy logging!

