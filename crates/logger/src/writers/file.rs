use crate::utils::strip_ansi;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// Writer that outputs to a file (ANSI codes stripped)
pub struct FileWriter {
    file: File,
}

impl FileWriter {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self { file })
    }
}

impl Write for FileWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let clean = strip_ansi(buf);
        self.file.write_all(&clean)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}
