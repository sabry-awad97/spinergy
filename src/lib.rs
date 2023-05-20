pub use colored::Color;
pub use error::{SpinnerError, SpinnerResult};
pub use spinner::stream::SpinnerStream;
pub use spinner::Spinner;

mod config;
mod error;
mod spinner;
