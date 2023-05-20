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
        if self.is_running() {
            return Err(SpinnerError::new("Spinner is already running"));
        }
        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn stop(&mut self) -> SpinnerResult<()> {
        if !self.is_running() {
            return Err(SpinnerError::new("Spinner is not running"));
        }
        self.state
            .channel
            .try_send(SpinnerMessage::Stop)
            .map_err(|_| SpinnerError::new("Failed to send message through channel"))?;
        self.running.store(false, Ordering::SeqCst);
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        if self.is_running() {
            self.stop().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let spinner = Spinner::new();
        assert_eq!(spinner.is_running(), false);
    }

    #[test]
    fn test_start_spinner() {
        let mut spinner = Spinner::new();
        let result = spinner.start();
        assert_eq!(result.is_ok(), true);
        assert_eq!(spinner.is_running(), true);
    }

    #[test]
    fn test_start_running_spinner() {
        let mut spinner = Spinner::new();
        spinner.running.store(true, Ordering::SeqCst);
        let result = spinner.start();
        assert_eq!(result.is_err(), true);
        assert_eq!(spinner.is_running(), true);
    }

    #[test]
    fn test_stop_spinner() {
        let mut spinner = Spinner::new();
        spinner.start().unwrap();
        let result = spinner.stop();
        assert_eq!(result.is_ok(), true);
        assert_eq!(spinner.is_running(), false);
    }

    #[test]
    fn test_stop_stopped_spinner() {
        let mut spinner = Spinner::new();
        let result = spinner.stop();
        assert_eq!(result.is_err(), true);
        assert_eq!(spinner.is_running(), false);
    }

    #[test]
    fn test_start_stop() {
        let mut spinner = Spinner::new();
        assert_eq!(spinner.start().is_ok(), true);
        assert_eq!(spinner.running.load(Ordering::SeqCst), true);
        assert_eq!(spinner.start().is_err(), true);
        assert_eq!(spinner.stop().is_ok(), true);
        assert_eq!(spinner.is_running(), false);
    }
}
