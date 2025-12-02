//! Logger tests
//!
//! Run with: `cargo test -p rust-telemetry -- --test-threads=1`
//! (tests share global state, so they must run serially)

use rust_telemetry::{clear_output, debug, error, info, set_output, warn};
use std::io::Cursor;
use std::sync::{Arc, Mutex};

/// A thread-safe buffer we can write to and read from
fn create_test_buffer() -> Arc<Mutex<Cursor<Vec<u8>>>> {
    Arc::new(Mutex::new(Cursor::new(Vec::new())))
}

/// Wrapper to make our buffer implement Write + Send
struct TestWriter(Arc<Mutex<Cursor<Vec<u8>>>>);

impl std::io::Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.lock().unwrap().flush()
    }
}

#[test]
fn test_debug_output() {
    let buffer = create_test_buffer();
    set_output(Box::new(TestWriter(buffer.clone())));

    debug("hello world");

    let output = buffer.lock().unwrap();
    let result = String::from_utf8_lossy(output.get_ref());

    assert!(result.contains("[DEBUG]"));
    assert!(result.contains("hello world"));

    clear_output();
}

#[test]
fn test_info_output() {
    let buffer = create_test_buffer();
    set_output(Box::new(TestWriter(buffer.clone())));

    info("server started");

    let output = buffer.lock().unwrap();
    let result = String::from_utf8_lossy(output.get_ref());

    assert!(result.contains("[INFO"));
    assert!(result.contains("server started"));

    clear_output();
}

#[test]
fn test_warn_output() {
    let buffer = create_test_buffer();
    set_output(Box::new(TestWriter(buffer.clone())));

    warn("memory low");

    let output = buffer.lock().unwrap();
    let result = String::from_utf8_lossy(output.get_ref());

    assert!(result.contains("[WARN"));
    assert!(result.contains("memory low"));

    clear_output();
}

#[test]
fn test_error_output() {
    let buffer = create_test_buffer();
    set_output(Box::new(TestWriter(buffer.clone())));

    error("connection failed");

    let output = buffer.lock().unwrap();
    let result = String::from_utf8_lossy(output.get_ref());

    assert!(result.contains("[ERROR]"));
    assert!(result.contains("connection failed"));

    clear_output();
}

#[test]
fn test_multiple_logs() {
    let buffer = create_test_buffer();
    set_output(Box::new(TestWriter(buffer.clone())));

    debug("first");
    info("second");
    warn("third");
    error("fourth");

    let output = buffer.lock().unwrap();
    let result = String::from_utf8_lossy(output.get_ref());

    assert!(result.contains("first"));
    assert!(result.contains("second"));
    assert!(result.contains("third"));
    assert!(result.contains("fourth"));

    clear_output();
}
