use crate::writers::StdoutWriter;
use std::io::Write;
use std::sync::{LazyLock, Mutex};
use std::time::SystemTime;

/// Global writer for log output (defaults to stdout)
static WRITER: LazyLock<Mutex<Box<dyn Write + Send>>> =
    LazyLock::new(|| Mutex::new(Box::new(StdoutWriter::new())));

/// Global timestamp override (None = use current time)
static TIMESTAMP: Mutex<Option<String>> = Mutex::new(None);

/// Set a custom output target for logs
pub fn set_output<W: Write + Send + 'static>(writer: W) {
    let mut w = WRITER.lock().unwrap();
    *w = Box::new(writer);
}

/// Clear custom output, revert to stdout
pub fn clear_output() {
    let mut w = WRITER.lock().unwrap();
    *w = Box::new(StdoutWriter::new());
}

/// Set a fixed timestamp for all logs (useful for testing)
pub fn set_timestamp(ts: &str) {
    let mut t = TIMESTAMP.lock().unwrap();
    *t = Some(ts.to_string());
}

/// Clear fixed timestamp, use current time
pub fn clear_timestamp() {
    let mut t = TIMESTAMP.lock().unwrap();
    *t = None;
}

/// Get timestamp (fixed or current time)
fn timestamp() -> String {
    if let Some(ts) = TIMESTAMP.lock().unwrap().as_ref() {
        return ts.clone();
    }

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();

    let secs = now.as_secs();
    let millis = now.subsec_millis();

    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, millis)
}

/// Write a log line to the configured output
pub fn write_log(level: &str, color: &str, message: &str) {
    let time = timestamp();
    let line = format!("{time} {color}{level}\x1b[0m {message}\n");

    let mut writer = WRITER.lock().unwrap();
    let _ = writer.write_all(line.as_bytes());
    let _ = writer.flush();
}
