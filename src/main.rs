mod logger;
fn main() {
    println!("Hello, world!");
    let logger = logger::get_logger();
    {
        let mut log_instance = logger.lock().unwrap();
        match log_instance.log(logger::LogLevel::Info, "This is an info message") {
            Ok(_) => println!("Log message written successfully"),
            Err(e) => println!("Error writing log message: {}", e),
        }

        match log_instance.log(logger::LogLevel::Error, "This is an error message") {
            Ok(_) => println!("Log message written successfully"),
            Err(e) => println!("Error writing log message: {}", e),
        }

        match log_instance.log(logger::LogLevel::Warning, "This is a warning message") {
            Ok(_) => println!("Log message written successfully"),
            Err(e) => println!("Error writing log message: {}", e),
        }

        match log_instance.log(logger::LogLevel::Debug, "This is a debug message") {
            Ok(_) => println!("Log message written successfully"),
            Err(e) => println!("Error writing log message: {}", e),
        }
    }
}
