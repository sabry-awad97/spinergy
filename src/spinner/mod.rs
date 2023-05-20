use self::{message::SpinnerMessage, state::SpinnerState};
use crate::{SpinnerError, SpinnerResult};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex,
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
    paused: Arc<(Mutex<bool>, Condvar)>,
    state: SpinnerState,
}

impl Spinner {
    pub fn new() -> Self {
        let running = Arc::new(AtomicBool::new(false));
        let paused = Arc::new((Mutex::new(false), Condvar::new()));

        let state = SpinnerState::new();
        Self {
            running,
            state,
            paused,
        }
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

    pub fn pause(&mut self) -> SpinnerResult<()> {
        if !self.is_running() {
            return Err(SpinnerError::new("Spinner is not running"));
        }
        let (lock, cvar) = &*self.paused;
        let mut paused = lock.lock().unwrap();
        if *paused {
            return Err(SpinnerError::new("Spinner is already paused"));
        }
        *paused = true;
        cvar.notify_one();
        Ok(())
    }

    pub fn resume(&mut self) -> SpinnerResult<()> {
        if !self.is_running() {
            return Err(SpinnerError::new("Spinner is not running"));
        }
        let (lock, cvar) = &*self.paused;
        let mut paused = lock.lock().unwrap();
        if !*paused {
            return Err(SpinnerError::new("Spinner is not paused"));
        }
        *paused = false;
        cvar.notify_one();
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

    #[test]
    fn test_pause_spinner() {
        let mut spinner = Spinner::new();
        spinner.start().unwrap();
        assert_eq!(spinner.pause().is_ok(), true);
    }

    #[test]
    fn test_pause_paused_spinner() {
        let mut spinner = Spinner::new();
        spinner.start().unwrap();
        spinner.pause().unwrap();
        assert_eq!(spinner.pause().is_err(), true);
    }

    #[test]
    fn test_pause_stopped_spinner() {
        let mut spinner = Spinner::new();
        assert_eq!(spinner.pause().is_err(), true);
    }

    #[test]
    fn test_resume_spinner() {
        let mut spinner = Spinner::new();
        spinner.start().unwrap();
        spinner.pause().unwrap();
        assert_eq!(spinner.resume().is_ok(), true);
    }

    #[test]
    fn test_resume_running_spinner() {
        let mut spinner = Spinner::new();
        spinner.start().unwrap();
        assert_eq!(spinner.resume().is_err(), true);
    }
    #[test]
    fn test_resume_unpaused_spinner() {
        let mut spinner = Spinner::new();
        spinner.start().unwrap();
        assert_eq!(spinner.resume().is_err(), true);
    }

    #[test]
    fn test_resume_stopped_spinner() {
        let mut spinner = Spinner::new();
        assert_eq!(spinner.resume().is_err(), true);
    }

    #[test]
    fn test_pause_resume_multiple_times() {
        let mut spinner = Spinner::new();
        assert!(spinner.start().is_ok());
        assert!(spinner.pause().is_ok());
        assert!(spinner.resume().is_ok());
        assert!(spinner.pause().is_ok());
        assert!(spinner.resume().is_ok());
        assert!(spinner.stop().is_ok());
    }

    #[test]
    fn test_resume_while_not_paused() {
        let mut spinner = Spinner::new();
        assert!(spinner.start().is_ok());
        assert!(spinner.resume().is_err());
        assert!(spinner.stop().is_ok());
    }

    #[test]
    fn test_drop() {
        let mut spinner = Spinner::new();
        assert!(spinner.start().is_ok());
        drop(spinner);
    }
}
