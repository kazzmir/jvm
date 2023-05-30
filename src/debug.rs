#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ({
        if cfg!(debug_assertions) {
            eprintln!("[{}:{}] {}", file!(), line!(), format_args!($($arg)*))
        }
    })
}

