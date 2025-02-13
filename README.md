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
  All log calls are guarded by a `Mutex`, ensuring that concurrent writes are safe and deterministic.

- **Comprehensive Testing**  
  A full test suite verifies log file creation, timestamp formatting, log level categorization, and message appending.

---

## 📝 Usage

### **Installation: Clone or Add as Dependency**

You can use the Singleton Logger in two ways:

#### **1️⃣ Clone the Repository Locally**
If you prefer to copy the code manually into your project:
```sh
git clone https://github.com/Tekunoroji/Singleton-collection.git
cd Singleton-collection
```
Then, include the logger module in your Rust project by copying the `st_logger` module into your source directory.

#### **2️⃣ Add as a Dependency in Cargo.toml**
To integrate it directly as a dependency, add the following to your `Cargo.toml`:

```toml
[dependencies]
singleton_logger = { git = "https://github.com/Tekunoroji/Singleton-collection.git", tag = "0.2" }
```

> 🔹 **Note:** Replace `"0.2"` with the latest available version.  
> To check available tags, run:
> ```sh
> git ls-remote --tags https://github.com/Tekunoroji/Singleton-collection.git
> ```

---

### **Logging via Macros**

Once configured, you can log messages using simple macros:

```rust
globaldeb!("Debug message: {}", some_debug_value);
globalinfo!("Info message: {}", some_info);
globalwarn!("Warning: {}", some_warning);
globalerror!("Error occurred: {}", some_error);
```

Each macro accesses the global singleton logger, formats the message with a timestamp and log level, and dispatches it to all registered writers (e.g., console and file).

---

### **Initialization**

The global logger is already pre-configured inside the crate:

- **Production Mode**  
  - Logs to `app.log` and the console.
  
- **Testing Mode**  
  - Logs to `test_app.log` and the console.

#### **If You Cloned the Repository Manually**
Include the logger modules manually in your `main.rs`:
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

#### **If You Added It as a Dependency**
Simply import the logger and use it in your `main.rs`:
```rust
use singleton_logger::*;

fn main() {
    globalinfo!("Logger successfully initialized!");
}
```

This ensures you can use the logger seamlessly across different projects.

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

## 🔄 Version Control & Updates

If you want to **use a specific version** of the logger, reference the corresponding Git tag in your `Cargo.toml`:

```toml
[dependencies]
singleton_logger = { git = "https://github.com/Tekunoroji/Singleton-collection.git", tag = "0.2" }
```

Or, to track the latest changes from a branch:

```toml
[dependencies]
singleton_logger = { git = "https://github.com/Tekunoroji/Singleton-collection.git", branch = "main" }
```

If you need a **specific commit**, reference it using `rev`:

```toml
[dependencies]
singleton_logger = { git = "https://github.com/Tekunoroji/Singleton-collection.git", rev = "abc123def456" }
```

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


