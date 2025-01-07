#[macro_export]
macro_rules! log_custom {
    ($fmt:expr, $color:expr) => {
       let fmt = format!($fmt);
        println!("{}{}{}", $color.0, fmt, $color.1);
    };
    ($fmt:expr, $color:expr, $($args:tt)*) => {
       let fmt = format!($fmt, $($args)*);
        println!("{}{}{}", $color.0, fmt, $color.1);
    };
}
#[macro_export]
macro_rules! log_info {
    ($fmt:expr) => {
       let fmt = format!($fmt);
        println!("{}{}{}", ansi::BOLD_GREEN.0, fmt, ansi::BOLD_GREEN.1);
    };
    ($fmt:expr, $($args:tt)*) => {
       let fmt = format!($fmt, $($args)*);
        println!("{}{}{}", ansi::BOLD_GREEN.0, fmt, ansi::BOLD_GREEN.1);
    };
}
#[macro_export]
macro_rules! log_warn {
    ($fmt:expr) => {
       let fmt = format!($fmt);
        println!("{}{}{}", ansi::BOLD_YELLOW.0, fmt, ansi::BOLD_YELLOW.1);
    };
    ($fmt:expr, $($args:tt)*) => {
       let fmt = format!($fmt, $($args)*);
        println!("{}{}{}", ansi::BOLD_YELLOW.0, fmt, ansi::BOLD_YELLOW.1);
    };
}
#[macro_export]
macro_rules! log_fatal {
    ($fmt:expr) => {
       let fmt = format!($fmt);
        println!("{}{}{}", ansi::BOLD_RED.0, fmt, ansi::BOLD_RED.1);
    };
    ($fmt:expr, $($args:tt)*) => {
       let fmt = format!($fmt, $($args)*);
        println!("{}{}{}", ansi::BOLD_RED.0, fmt, ansi::BOLD_RED.1);
    };
}
