pub use colored::Color;
pub use error::{SpinnerError, SpinnerResult};
pub use spinner::alignment::Alignment;
pub use spinner::builtins::SpinnerStyle;
pub use spinner::event::Event;
pub use spinner::stream::SpinnerStream;
pub use spinner::Spinner;

mod config;
mod error;
mod event_emitter;
mod spinner;
