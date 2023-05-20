use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum SpinnerStream {
    Stdout,
    Stderr,
}

impl Default for SpinnerStream {
    fn default() -> Self {
        Self::Stdout
    }
}

impl Write for SpinnerStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            SpinnerStream::Stdout => io::stdout().write(buf),
            SpinnerStream::Stderr => io::stderr().write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            SpinnerStream::Stdout => io::stdout().flush(),
            SpinnerStream::Stderr => io::stderr().flush(),
        }
    }
}
