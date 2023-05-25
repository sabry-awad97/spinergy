use colored::Color;

use crate::SpinnerResult;

use super::{alignment::Alignment, builtins::SpinnerStyle};

#[derive(Debug, Clone)]
pub enum SpinnerMessage {
    Stop,
    Update(SpinnerResult<UpdateMessage>),
}

#[derive(Debug, Clone)]
pub enum UpdateMessage {
    Message(String),
    Style(SpinnerStyle),
    Alignment(Alignment),
    Colors(Option<Color>, Option<Color>),
}
