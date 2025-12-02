use crate::utils::strip_ansi;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

/// Writer that stores output in memory (ANSI codes stripped)
/// Useful for testing
#[derive(Clone, Default)]
pub struct MemoryWriter {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl MemoryWriter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the contents as a string
    pub fn contents(&self) -> String {
        let data = self.buffer.lock().unwrap();
        String::from_utf8_lossy(&data).to_string()
    }

    /// Clear the buffer
    pub fn clear(&self) {
        self.buffer.lock().unwrap().clear();
    }

    /// Get a boxed writer for use with set_output
    pub fn writer(&self) -> Box<dyn Write + Send> {
        Box::new(self.clone())
    }
}

impl Write for MemoryWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let clean = strip_ansi(buf);
        self.buffer.lock().unwrap().extend_from_slice(&clean);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
