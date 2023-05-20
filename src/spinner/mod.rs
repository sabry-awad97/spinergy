use self::{message::SpinnerMessage, state::SpinnerState};
use crate::{SpinnerError, SpinnerResult};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

mod alignment;
mod builtins;
mod channel;
mod event;
mod message;
mod state;
mod stream;

pub struct Spinner {
    running: Arc<AtomicBool>,
    state: SpinnerState,
}

impl Spinner {
    pub fn new() -> Self {
        let running = Arc::new(AtomicBool::new(false));
        let state = SpinnerState::new();
        Self { running, state }
    }

    pub fn start(&mut self) -> SpinnerResult<()> {
        if self.running.load(Ordering::SeqCst) {
            return Err(SpinnerError::new("Spinner is already running"));
        }
        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn stop(&mut self) -> SpinnerResult<()> {
        if self.state.channel.try_send(SpinnerMessage::Stop).is_err() {
            return Err(SpinnerError::new("Failed to send message through channel"));
        }
        self.running.store(false, Ordering::SeqCst);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let spinner = Spinner::new();
        assert_eq!(spinner.running.load(Ordering::SeqCst), false);
    }

    #[test]
    fn test_start_stop() {
        let mut spinner = Spinner::new();
        assert_eq!(spinner.start().is_ok(), true);
        assert_eq!(spinner.running.load(Ordering::SeqCst), true);
        assert_eq!(spinner.start().is_err(), true);
        assert_eq!(spinner.stop().is_ok(), true);
        assert_eq!(spinner.running.load(Ordering::SeqCst), false);
    }
}
