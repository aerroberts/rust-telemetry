//! # rust-telemetry
//!
//! A lightweight, ergonomic logging library for Rust applications.
//!
//! ## Quick Start
//!
//! ```rust
//! use rust_telemetry::{init, info, warn, error, Level};
//!
//! fn main() {
//!     // Initialize with default settings (Info level)
//!     init();
//!
//!     info!("Application started");
//!     warn!("This is a warning");
//!     error!("Something went wrong: {}", "connection failed");
//! }
//! ```

use std::fmt;
use std::io::{self, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::SystemTime;

/// Global log level filter
static MAX_LEVEL: AtomicUsize = AtomicUsize::new(Level::Info as usize);

/// Log levels in order of severity (lowest to highest)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(usize)]
pub enum Level {
    /// Fine-grained debugging information
    Trace = 0,
    /// Debugging information
    Debug = 1,
    /// General information
    Info = 2,
    /// Warning conditions
    Warn = 3,
    /// Error conditions
    Error = 4,
    /// Logging disabled
    Off = 5,
}

impl Level {
    /// Returns the string representation of the log level
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
            Level::Off => "OFF",
        }
    }

    /// Returns ANSI color code for the level
    pub fn color(&self) -> &'static str {
        match self {
            Level::Trace => "\x1b[35m", // Magenta
            Level::Debug => "\x1b[36m", // Cyan
            Level::Info => "\x1b[32m",  // Green
            Level::Warn => "\x1b[33m",  // Yellow
            Level::Error => "\x1b[31m", // Red
            Level::Off => "",
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Level {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "TRACE" => Ok(Level::Trace),
            "DEBUG" => Ok(Level::Debug),
            "INFO" => Ok(Level::Info),
            "WARN" | "WARNING" => Ok(Level::Warn),
            "ERROR" => Ok(Level::Error),
            "OFF" => Ok(Level::Off),
            _ => Err(()),
        }
    }
}

/// A log record containing all information about a single log event
#[derive(Debug)]
pub struct Record<'a> {
    pub level: Level,
    pub message: &'a str,
    pub module_path: Option<&'a str>,
    pub file: Option<&'a str>,
    pub line: Option<u32>,
}

impl<'a> Record<'a> {
    pub fn new(level: Level, message: &'a str) -> Self {
        Self {
            level,
            message,
            module_path: None,
            file: None,
            line: None,
        }
    }

    pub fn module_path(mut self, path: &'a str) -> Self {
        self.module_path = Some(path);
        self
    }

    pub fn file(mut self, file: &'a str) -> Self {
        self.file = Some(file);
        self
    }

    pub fn line(mut self, line: u32) -> Self {
        self.line = Some(line);
        self
    }
}

/// Initialize the logger with default settings (Info level)
pub fn init() {
    set_max_level(Level::Info);
}

/// Initialize the logger with a specific log level
pub fn init_with_level(level: Level) {
    set_max_level(level);
}

/// Set the maximum log level filter
pub fn set_max_level(level: Level) {
    MAX_LEVEL.store(level as usize, Ordering::SeqCst);
}

/// Get the current maximum log level
pub fn max_level() -> Level {
    match MAX_LEVEL.load(Ordering::Relaxed) {
        0 => Level::Trace,
        1 => Level::Debug,
        2 => Level::Info,
        3 => Level::Warn,
        4 => Level::Error,
        _ => Level::Off,
    }
}

/// Check if a log level is enabled
#[inline]
pub fn log_enabled(level: Level) -> bool {
    level as usize >= MAX_LEVEL.load(Ordering::Relaxed)
}

/// Format and write a log record
pub fn log(record: &Record) {
    if !log_enabled(record.level) {
        return;
    }

    let reset = "\x1b[0m";
    let timestamp = format_timestamp();

    let location = match (record.file, record.line) {
        (Some(file), Some(line)) => format!(" {}:{}", file, line),
        _ => String::new(),
    };

    let output = format!(
        "{}{}{:<5}{} {}{}\n",
        timestamp,
        record.level.color(),
        record.level.as_str(),
        reset,
        record.message,
        location,
    );

    // Write errors and warnings to stderr, everything else to stdout
    let result = if record.level >= Level::Warn {
        io::stderr().write_all(output.as_bytes())
    } else {
        io::stdout().write_all(output.as_bytes())
    };

    if let Err(e) = result {
        eprintln!("Failed to write log: {}", e);
    }
}

fn format_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();

    let secs = now.as_secs();
    let millis = now.subsec_millis();

    // Simple timestamp format: HH:MM:SS.mmm (UTC)
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!("{:02}:{:02}:{:02}.{:03} ", hours, minutes, seconds, millis)
}

// ============================================================================
// Logging Macros
// ============================================================================

/// Log a message at the trace level
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        if $crate::log_enabled($crate::Level::Trace) {
            let msg = format!($($arg)*);
            let record = $crate::Record::new($crate::Level::Trace, &msg)
                .file(file!())
                .line(line!());
            $crate::log(&record);
        }
    };
}

/// Log a message at the debug level
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if $crate::log_enabled($crate::Level::Debug) {
            let msg = format!($($arg)*);
            let record = $crate::Record::new($crate::Level::Debug, &msg)
                .file(file!())
                .line(line!());
            $crate::log(&record);
        }
    };
}

/// Log a message at the info level
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if $crate::log_enabled($crate::Level::Info) {
            let msg = format!($($arg)*);
            let record = $crate::Record::new($crate::Level::Info, &msg)
                .file(file!())
                .line(line!());
            $crate::log(&record);
        }
    };
}

/// Log a message at the warn level
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        if $crate::log_enabled($crate::Level::Warn) {
            let msg = format!($($arg)*);
            let record = $crate::Record::new($crate::Level::Warn, &msg)
                .file(file!())
                .line(line!());
            $crate::log(&record);
        }
    };
}

/// Log a message at the error level
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        if $crate::log_enabled($crate::Level::Error) {
            let msg = format!($($arg)*);
            let record = $crate::Record::new($crate::Level::Error, &msg)
                .file(file!())
                .line(line!());
            $crate::log(&record);
        }
    };
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_ordering() {
        assert!(Level::Trace < Level::Debug);
        assert!(Level::Debug < Level::Info);
        assert!(Level::Info < Level::Warn);
        assert!(Level::Warn < Level::Error);
        assert!(Level::Error < Level::Off);
    }

    #[test]
    fn test_level_from_str() {
        assert_eq!("trace".parse::<Level>(), Ok(Level::Trace));
        assert_eq!("DEBUG".parse::<Level>(), Ok(Level::Debug));
        assert_eq!("Info".parse::<Level>(), Ok(Level::Info));
        assert_eq!("WARN".parse::<Level>(), Ok(Level::Warn));
        assert_eq!("warning".parse::<Level>(), Ok(Level::Warn));
        assert_eq!("error".parse::<Level>(), Ok(Level::Error));
        assert_eq!("off".parse::<Level>(), Ok(Level::Off));
        assert!("invalid".parse::<Level>().is_err());
    }

    #[test]
    fn test_set_max_level() {
        set_max_level(Level::Debug);
        assert_eq!(max_level(), Level::Debug);
        assert!(log_enabled(Level::Debug));
        assert!(log_enabled(Level::Info));
        assert!(!log_enabled(Level::Trace));

        set_max_level(Level::Error);
        assert_eq!(max_level(), Level::Error);
        assert!(log_enabled(Level::Error));
        assert!(!log_enabled(Level::Warn));
    }

    #[test]
    fn test_record_builder() {
        let record = Record::new(Level::Info, "test message")
            .file("test.rs")
            .line(42)
            .module_path("test::module");

        assert_eq!(record.level, Level::Info);
        assert_eq!(record.message, "test message");
        assert_eq!(record.file, Some("test.rs"));
        assert_eq!(record.line, Some(42));
        assert_eq!(record.module_path, Some("test::module"));
    }
}

