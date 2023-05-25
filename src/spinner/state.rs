use std::io::Write;
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicBool, Arc, Condvar, Mutex};

use super::{channel::Channel, message::UpdateMessage};
use crate::{spinner::message::SpinnerMessage, SpinnerError, SpinnerResult, SpinnerStream};

#[derive(Clone)]
pub struct SpinnerState {
    pub channel: Channel<SpinnerMessage>,
    pub output: Arc<Mutex<SpinnerStream>>,
}

impl SpinnerState {
    pub fn new() -> Self {
        let channel = Channel::new();

        let stream = SpinnerStream::default();
        let output = Arc::new(Mutex::new(stream));

        Self { channel, output }
    }

    pub fn update(&mut self, message: UpdateMessage) -> SpinnerResult<()> {
        self.channel
            .try_send(SpinnerMessage::Update(Ok(message)))
            .map_err(|_| "Failed to send message through channel".into())
    }

    pub fn spin(
        &mut self,
        running: Arc<AtomicBool>,
        paused: Arc<(Mutex<bool>, Condvar)>,
    ) -> SpinnerResult<()> {
        write!(self.output.lock().unwrap(), "\x1B[?25l")
            .map_err(|e| SpinnerError::new(&e.to_string()))?; // hide cursor

        loop {
            if !running.load(Ordering::SeqCst) {
                break;
            }

            let (lock, cvar) = &*paused;
            let mut paused = lock.lock().unwrap();
            while *paused {
                paused = cvar.wait(paused).unwrap();
            }

            if let Ok(spin_message) = self.channel.try_receive() {
                match spin_message {
                    SpinnerMessage::Stop => {
                        if self.channel.try_send(SpinnerMessage::Stop).is_err() {
                            return Err("Failed to send message through channel".into());
                        }
                        return Err("Spinner stopped".into());
                    }
                    SpinnerMessage::Update(result) => match result {
                        Ok(UpdateMessage::Text(message)) => {}
                        Err(_) => return Err("Failed to receive update message".into()),
                    },
                }
            }
        }
        Ok(())
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

    #[test]
    fn test_spin() {
        let mut state = SpinnerState::new();
        let running = Arc::new(AtomicBool::new(true));
        let paused = Arc::new((Mutex::new(false), Condvar::new()));
        assert!(state.spin(running.clone(), paused.clone()).is_ok());
    }
}
