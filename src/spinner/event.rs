use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Event {
    Start,
    Stop,
    Pause,
    Resume,
}

impl Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => write!(f, "Start"),
            Self::Stop => write!(f, "Stop"),
            Self::Pause => write!(f, "Pause"),
            Self::Resume => write!(f, "Resume"),
        }
    }
}
