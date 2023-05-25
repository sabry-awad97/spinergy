use crate::SpinnerResult;

use super::builtins::SpinnerStyle;

#[derive(Debug, Clone)]
pub enum SpinnerMessage {
    Stop,
    Update(SpinnerResult<UpdateMessage>),
}

#[derive(Debug, Clone)]
pub enum UpdateMessage {
    Message(String),
    Style(SpinnerStyle),
}
