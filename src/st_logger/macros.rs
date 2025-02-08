#[macro_export]
macro_rules! globaldeb {
    ($($arg:tt)*) => {{
        let logger = $crate::st_logger::logger::get_logger().lock().unwrap();
        logger.log($crate::st_logger::logger::LogLevel::Debug, &format!($($arg)*))
              .unwrap();
    }};
}

#[macro_export]
macro_rules! globalinfo {
    ($($arg:tt)*) => {{
        let logger = $crate::st_logger::logger::get_logger().lock().unwrap();
        logger.log($crate::st_logger::logger::LogLevel::Info, &format!($($arg)*))
               .unwrap();
    }};
}

#[macro_export]
macro_rules! globalwarn {
    ($($arg:tt)*) => {{
        let logger = $crate::st_logger::logger::get_logger().lock().unwrap();
        logger.log($crate::st_logger::logger::LogLevel::Warning, &format!($($arg)*))
               .unwrap();
    }};
}

#[macro_export]
macro_rules! globalerror {
    ($($arg:tt)*) => {{
        let logger = $crate::st_logger::logger::get_logger().lock().unwrap();
        logger.log($crate::st_logger::logger::LogLevel::Error, &format!($($arg)*))
               .unwrap();
    }};
}
