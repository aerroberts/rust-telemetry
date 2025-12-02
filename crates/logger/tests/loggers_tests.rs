use rust_telemetry::{clear_output, debug, error, info, set_output, warn};
use std::io::Write;
use std::sync::{Arc, Mutex};

/// Thread-safe buffer for capturing log output
struct TestBuffer(Arc<Mutex<Vec<u8>>>);

impl TestBuffer {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(Vec::new())))
    }

    fn writer(&self) -> TestWriter {
        TestWriter(self.0.clone())
    }

    fn contents(&self) -> String {
        let data = self.0.lock().unwrap();
        String::from_utf8_lossy(&data).to_string()
    }
}

struct TestWriter(Arc<Mutex<Vec<u8>>>);

impl Write for TestWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn test_debug_output() {
    let buffer = TestBuffer::new();
    set_output(Box::new(buffer.writer()));

    debug("hello world");

    assert!(buffer.contents().contains("[DEBUG]"));
    assert!(buffer.contents().contains("hello world"));
    clear_output();
}

#[test]
fn test_info_output() {
    let buffer = TestBuffer::new();
    set_output(Box::new(buffer.writer()));

    info("server started");

    assert!(buffer.contents().contains("[INFO"));
    assert!(buffer.contents().contains("server started"));
    clear_output();
}

#[test]
fn test_warn_output() {
    let buffer = TestBuffer::new();
    set_output(Box::new(buffer.writer()));

    warn("memory low");

    assert!(buffer.contents().contains("[WARN"));
    assert!(buffer.contents().contains("memory low"));
    clear_output();
}

#[test]
fn test_error_output() {
    let buffer = TestBuffer::new();
    set_output(Box::new(buffer.writer()));

    error("connection failed");

    assert!(buffer.contents().contains("[ERROR]"));
    assert!(buffer.contents().contains("connection failed"));
    clear_output();
}

#[test]
fn test_multiple_logs() {
    let buffer = TestBuffer::new();
    set_output(Box::new(buffer.writer()));

    debug("first");
    info("second");
    warn("third");
    error("fourth");

    let output = buffer.contents();
    assert!(output.contains("first"));
    assert!(output.contains("second"));
    assert!(output.contains("third"));
    assert!(output.contains("fourth"));
    clear_output();
}
