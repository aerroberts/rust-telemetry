use std::io::Write;
use std::sync::Mutex;

/// Global writer for log output
static WRITER: Mutex<Option<Box<dyn Write + Send>>> = Mutex::new(None);

/// Set a custom output target for logs
pub fn set_output(writer: Box<dyn Write + Send>) {
    let mut w = WRITER.lock().unwrap();
    *w = Some(writer);
}

/// Clear custom output, revert to default (stdout/stderr)
pub fn clear_output() {
    let mut w = WRITER.lock().unwrap();
    *w = None;
}

/// Write a log line to the configured output (or default)
pub fn write_log(level: &str, color: &str, message: &str) {
    let line = format!("{color}[{level}]\x1b[0m {message}\n");

    let mut writer = WRITER.lock().unwrap();

    match writer.as_mut() {
        Some(w) => {
            let _ = w.write_all(line.as_bytes());
            let _ = w.flush();
        }
        None => {
            // No output
        }
    }
}
