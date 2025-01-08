#[macro_export]
macro_rules! log_color {
    ($fmt:expr, $color:expr) => {
        let fmt = format!($fmt);
        println!("{}{}{}", $color.prefix, fmt, $color.suffix);
    };
    ($fmt:expr, $color:expr, $($args:tt)*) => {
        let fmt = format!($fmt, $($args)*);
        println!("{}{}{}", $color.prefix, fmt, $color.suffix);
    };
}

#[macro_export]
macro_rules! log_info {
    ($fmt:expr) => {
        let fmt = format!($fmt);
        println!("{}{}{}", ansi::BOLD_BLUE.prefix, fmt, ansi::BOLD_BLUE.suffix);
    };
    ($fmt:expr, $($args:tt)*) => {
        let fmt = format!($fmt, $($args)*);
        println!("{}{}{}", ansi::BOLD_BLUE.prefix, fmt, ansi::BOLD_BLUE.suffix);
    };
}

#[macro_export]
macro_rules! log_event {
    ($fmt:expr) => {
        let fmt = format!($fmt);
        println!("{}{}{}", ansi::BOLD_GREEN.prefix, fmt, ansi::BOLD_GREEN.suffix);
    };
    ($fmt:expr, $($args:tt)*) => {
        let fmt = format!($fmt, $($args)*);
        println!("{}{}{}", ansi::BOLD_GREEN.prefix, fmt, ansi::BOLD_GREEN.suffix);
    };
}

#[macro_export]
macro_rules! log_warn {
    ($fmt:expr) => {
        let fmt = format!($fmt);
        println!("{}{}{}", ansi::BOLD_YELLOW.prefix, fmt, ansi::BOLD_YELLOW.suffix);
    };
    ($fmt:expr, $($args:tt)*) => {
        let fmt = format!($fmt, $($args)*);
        println!("{}{}{}", ansi::BOLD_YELLOW.prefix, fmt, ansi::BOLD_YELLOW.suffix);
    };
}

#[macro_export]
macro_rules! log_fatal {
    ($fmt:expr) => {
        let fmt = format!($fmt);
        println!("{}{}{}", ansi::BOLD_RED.prefix, fmt, ansi::BOLD_RED.suffix);
    };
    ($fmt:expr, $($args:tt)*) => {
        let fmt = format!($fmt, $($args)*);
        println!("{}{}{}", ansi::BOLD_RED.prefix, fmt, ansi::BOLD_RED.suffix);
    };
}
