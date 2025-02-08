#[macro_export]
macro_rules! globaldeb {
    ($($arg:tt)*) => {{
        let logger = $crate::logger::get_logger().lock().unwrap();
        logger.log($crate::logger::LogLevel::Debug, &format!($($arg)*))
              .unwrap();
    }};
}

#[macro_export]
macro_rules! globalinfo {
    ($($arg:tt)*) => {{
        let logger = $crate::logger::get_logger().lock().unwrap();
        logger.log($crate::logger::LogLevel::Info, &format!($($arg)*))
               .unwrap();
    }};
}

#[macro_export]
macro_rules! globalwarn {
    ($($arg:tt)*) => {{
        let logger = $crate::logger::get_logger().lock().unwrap();
        logger.log($crate::logger::LogLevel::Warning, &format!($($arg)*))
               .unwrap();
    }};
}

#[macro_export]
macro_rules! globalerror {
    ($($arg:tt)*) => {{
        let logger = $crate::logger::get_logger().lock().unwrap();
        logger.log($crate::logger::LogLevel::Error, &format!($($arg)*))
               .unwrap();
    }};
}
