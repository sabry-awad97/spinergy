use super::{channel::Channel, message::UpdateMessage};
use crate::{spinner::message::SpinnerMessage, SpinnerError, SpinnerResult};

#[derive(Clone)]
pub struct SpinnerState {
    pub channel: Channel<SpinnerMessage>,
}

impl SpinnerState {
    pub fn new() -> Self {
        let channel = Channel::new();
        Self { channel }
    }

    pub fn update(&mut self, message: UpdateMessage) -> SpinnerResult<()> {
        self.channel
            .try_send(SpinnerMessage::Update(Ok(message)))
            .map_err(|_| SpinnerError::new("Failed to send message through channel"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spinner::message::SpinnerMessage;

    #[test]
    fn test_update() {
        let mut state = SpinnerState::new();
        let message = UpdateMessage::Text("test".to_owned());
        state.update(message.clone()).unwrap();
        let received_message = state.channel.try_receive().unwrap();
        assert!(matches!(received_message, SpinnerMessage::Update(Ok(_))));
        if let SpinnerMessage::Update(Ok(received_update)) = received_message {
            assert!(matches!(received_update, UpdateMessage::Text(_)))
        }
    }
}
