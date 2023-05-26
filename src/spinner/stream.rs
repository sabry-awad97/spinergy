use std::io::{self, Write};

#[derive(Debug, Clone)]
pub enum SpinnerStream {
    Stdout,
    Stderr,
}

impl<'a> Into<SpinnerStream> for &'a str {
    fn into(self) -> SpinnerStream {
        match self {
            "stdout" => SpinnerStream::Stdout,
            "stderr" => SpinnerStream::Stderr,
            _ => panic!("Invalid spinner stream"),
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_stream_default() {
        let stream = SpinnerStream::default();
        assert!(matches!(stream, SpinnerStream::Stdout));
    }

    #[test]
    fn test_spinner_stream_write_stdout() {
        let mut stream = SpinnerStream::Stdout;
        let message = "Hello, world!";
        assert_eq!(stream.write(message.as_bytes()).unwrap(), message.len());
    }

    #[test]
    fn test_spinner_stream_write_stderr() {
        let mut stream = SpinnerStream::Stderr;
        let message = "Hello, world!";
        assert_eq!(stream.write(message.as_bytes()).unwrap(), message.len());
    }

    #[test]
    fn test_spinner_stream_flush_stdout() {
        let mut stream = SpinnerStream::Stdout;
        assert!(stream.flush().is_ok());
    }

    #[test]
    fn test_spinner_stream_flush_stderr() {
        let mut stream = SpinnerStream::Stderr;
        assert!(stream.flush().is_ok());
    }

    #[test]
    fn test_str_into_spinner_stream_valid() {
        let stdout: SpinnerStream = "stdout".into();
        assert!(matches!(stdout, SpinnerStream::Stdout));

        let stderr: SpinnerStream = "stderr".into();
        assert!(matches!(stderr, SpinnerStream::Stderr));
    }

    #[test]
    #[should_panic]
    fn test_str_into_spinner_stream_invalid() {
        // Test an invalid stream, expecting a panic
        let _ = Into::<SpinnerStream>::into("invalid");
    }
}
