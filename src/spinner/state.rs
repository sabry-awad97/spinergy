use super::channel::Channel;
use crate::spinner::message::SpinnerMessage;

#[derive(Clone)]
pub struct SpinnerState {
    pub channel: Channel<SpinnerMessage>,
}

impl SpinnerState {
    pub fn new() -> Self {
        let channel = Channel::new();
        Self { channel }
    }
}
