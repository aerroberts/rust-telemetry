use crate::config::write_log;

/// Log a debug message (cyan)
pub fn debug(message: &str) {
    write_log("debg", "\x1b[36m", message);
}

/// Log an info message (green)
pub fn info(message: &str) {
    write_log("info", "\x1b[32m", message);
}

/// Log a warning message (yellow)
pub fn warn(message: &str) {
    write_log("warn", "\x1b[33m", message);
}

/// Log an error message (red)
pub fn error(message: &str) {
    write_log("errr", "\x1b[31m", message);
}
