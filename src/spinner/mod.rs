use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{SpinnerError, SpinnerResult};

mod alignment;
mod builtins;
mod channel;
mod event;
mod message;
mod state;
mod stream;

pub struct Spinner {
    running: Arc<AtomicBool>,
}

impl Spinner {
    pub fn new() -> Self {
        let running = Arc::new(AtomicBool::new(false));
        Self { running }
    }

    pub fn start(&mut self) -> SpinnerResult<()> {
        if self.running.load(Ordering::SeqCst) {
            return Err(SpinnerError::new("Spinner is already running"));
        }
        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn stop(&mut self) -> SpinnerResult<()> {
        self.running.store(false, Ordering::SeqCst);
        Ok(())
    }
}
