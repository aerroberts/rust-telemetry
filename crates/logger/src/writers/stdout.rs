use std::io::{self, Write};

/// Writer that outputs to stdout
#[derive(Default)]
pub struct StdoutWriter;

impl StdoutWriter {
    pub fn new() -> Self {
        Self
    }
}

impl Write for StdoutWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::stdout().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}
