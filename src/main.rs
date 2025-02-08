mod composite_writer;
mod console_writer;
mod file_writer;
mod log_writer;
mod logger;
mod macros;
fn main() {
    globaldeb!("Hello, world!");
    globalinfo!("This is an info log");
    globalwarn!("This is a warning log");
    globalerror!("This is an error log");
}
