mod st_logger;
fn main() {
    globaldeb!("Hello, world!");
    globalinfo!("This is an info log");
    globalwarn!("This is a warning log");
    globalerror!("This is an error log");
}
