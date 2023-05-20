use crate::SpinnerResult;

#[derive(Debug, Clone)]
pub enum SpinnerMessage {
    Stop,
    Update(SpinnerResult<UpdateMessage>),
}

#[derive(Debug, Clone)]
pub enum UpdateMessage {
    Text(String),
}
