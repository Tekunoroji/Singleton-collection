# 🦀 Rust Singleton Logger

A simple, **thread-safe singleton logger** implemented in Rust using `once_cell` and a `Mutex`. This logger provides:

✅ **Colored messages** in the terminal
✅ **Plain-text logs** stored in a file
✅ **Flexible, easy-to-adapt** design for future projects

---

## 🚀 Features

- **🛠 Singleton** – Uses a static `LOGGER` (`Lazy<Mutex<Logger>>`) for global access.
- **🔒 Thread-safe** – Ensures concurrent log calls are written safely using a `Mutex`.
- **🎨 Colored Terminal Output** – Utilizes the `colored` crate for color-coded log levels.
- **📝 File Logging** – Automatically creates and appends logs to `app.log` or `test_app.log`.
- **🧪 Comprehensive Tests** – Includes a test suite covering timestamp format, log levels, and file handling.

---

✅ Running Tests
Run all tests using Cargo:

````
cargo test
````

Tests ensure:
✔ Log files are created correctly.
✔ Timestamps are properly formatted.
✔ Logs are correctly appended and categorized by log level.

---

🔮 Roadmap & Extensions
🔹 Async Logging – Consider tokio or channels for non-blocking writes.
🔹 Integration with log or tracing – Implement the Log trait or use a Subscriber.
🔹 Log Rotation – Add size-based or timed log rotation for production use.
