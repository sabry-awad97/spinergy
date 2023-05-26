use colored::Color;

use self::{
    alignment::Alignment, builtins::SpinnerStyle, event::Event, message::UpdateMessage,
    state::SpinnerState, stream::SpinnerStream,
};
use crate::{event_emitter::EventEmitter, SpinnerError, SpinnerResult};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

pub mod alignment;
mod builtins;
mod channel;
pub mod event;
mod message;
mod state;
pub mod stream;

pub struct Spinner {
    emitter: EventEmitter,
    running: Arc<AtomicBool>,
    paused: Arc<(Mutex<bool>, Condvar)>,
    state: SpinnerState,
    start_time: Option<Instant>,
    pause_start_time: Option<Instant>,
    pause_elapsed: Duration,
}

impl Spinner {
    pub fn new(message: impl Into<String>) -> Self {
        let running = Arc::new(AtomicBool::new(false));
        let paused = Arc::new((Mutex::new(false), Condvar::new()));

        let state = SpinnerState::new(message);

        let emitter = EventEmitter::new();

        let start_time = None;
        let pause_start_time = None;
        let pause_elapsed = Duration::from_secs(0);

        Self {
            emitter,
            running,
            state,
            paused,
            start_time,
            pause_start_time,
            pause_elapsed,
        }
    }

    #[allow(unused)]
    fn on<F, T>(&mut self, event_type: Event, mut listener: F)
    where
        F: FnMut(&[T]) + Sync + Send + 'static,
        T: 'static + Copy,
    {
        let callback = move |args: &[Box<dyn std::any::Any>]| {
            let typed_args: Vec<T> = args
                .iter()
                .map(|arg| *arg.downcast_ref().unwrap())
                .collect();
            listener(&typed_args);
        };

        let event_name = &event_type.to_string();
        self.emitter.on(event_name, callback);
    }

    pub fn start(&mut self) -> SpinnerResult<()> {
        if self.is_running() {
            return Err(SpinnerError::new("Spinner is already running"));
        }
        self.start_time = Some(Instant::now());
        self.emitter.emit(&Event::Start.to_string(), &[]);
        let running = self.running.clone();
        let paused = self.paused.clone();
        let mut state = self.state.clone();
        thread::spawn(move || state.spin(running, paused));
        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn on_start<F>(&mut self, mut listener: F)
    where
        F: FnMut() + Sync + Send + 'static,
    {
        let callback = move |_: &[Box<dyn std::any::Any>]| {
            listener();
        };

        self.emitter.on(&Event::Start.to_string(), callback);
    }

    pub fn stop(&mut self) -> SpinnerResult<()> {
        if !self.is_running() {
            return Err(SpinnerError::new("Spinner is not running"));
        }
        self.state.stop()?;
        self.running.store(false, Ordering::SeqCst);

        let mut elapsed = Duration::from_secs(0);
        if let Some(start_time) = self.start_time {
            elapsed = start_time.elapsed();
        }

        self.start_time = None;

        self.emitter
            .emit(&Event::Stop.to_string(), &[Box::new(elapsed)]);
        Ok(())
    }

    pub fn on_stop<F>(&mut self, mut listener: F)
    where
        F: FnMut(Duration) + Sync + Send + 'static,
    {
        let callback = move |args: &[Box<dyn std::any::Any>]| {
            if let Some(arg) = args.first() {
                if let Some(duration) = arg.downcast_ref::<Duration>() {
                    listener(*duration);
                }
            }
        };

        self.emitter.on(&Event::Stop.to_string(), callback);
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

        self.pause_start_time = Some(Instant::now());

        let elapsed = self.start_time.map(|start_time| start_time.elapsed());

        self.emitter
            .emit(&Event::Pause.to_string(), &[Box::new(elapsed)]);
        Ok(())
    }

    pub fn on_pause<F>(&mut self, mut listener: F)
    where
        F: FnMut(Option<Duration>) + Sync + Send + 'static,
    {
        let callback = move |args: &[Box<dyn std::any::Any>]| {
            if let Some(arg) = args.first() {
                if let Some(duration) = arg.downcast_ref::<Option<Duration>>() {
                    listener(*duration);
                }
            }
        };

        self.emitter.on(&Event::Pause.to_string(), callback);
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

        if let Some(pause_elapsed) = self.pause_start_time {
            self.pause_elapsed += pause_elapsed.elapsed();
        }

        self.pause_start_time = None;

        self.emitter
            .emit(&Event::Resume.to_string(), &[Box::new(self.pause_elapsed)]);
        Ok(())
    }

    pub fn on_resume<F>(&mut self, mut listener: F)
    where
        F: FnMut(Duration) + Sync + Send + 'static,
    {
        let callback = move |args: &[Box<dyn std::any::Any>]| {
            if let Some(arg) = args.first() {
                if let Some(duration) = arg.downcast_ref::<Duration>() {
                    listener(*duration);
                }
            }
        };

        self.emitter.on(&Event::Resume.to_string(), callback);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn set_message<T>(&mut self, message: T) -> SpinnerResult<()>
    where
        T: Into<String>,
    {
        self.state.update(UpdateMessage::Message(message.into()))
    }

    pub fn set_style(&mut self, style: impl Into<SpinnerStyle>) -> SpinnerResult<()> {
        self.state.update(UpdateMessage::Style(style.into()))
    }

    pub fn set_color_scheme<U>(
        &mut self,
        style_color: U,
        message_color: U,
        dots_color: U,
    ) -> SpinnerResult<()>
    where
        U: Into<Option<Color>>,
    {
        self.state.update(UpdateMessage::Colors(
            style_color.into(),
            message_color.into(),
            dots_color.into(),
        ))
    }

    pub fn set_reverse(&mut self, reverse: bool) -> SpinnerResult<()> {
        self.state.set_reverse(reverse);
        Ok(())
    }

    pub fn set_alignment<T>(&mut self, alignment: T) -> SpinnerResult<()>
    where
        T: Into<Alignment>,
    {
        self.state
            .update(UpdateMessage::Alignment(alignment.into()))
    }

    pub fn set_fps<V>(&mut self, fps: V) -> SpinnerResult<()>
    where
        V: Into<f64>,
    {
        self.state
            .update(UpdateMessage::FramesPerSecond(fps.into()))
    }

    pub fn set_speed<V>(&mut self, rpm: V) -> SpinnerResult<()>
    where
        V: Into<f64>,
    {
        self.state.update(UpdateMessage::Speed(rpm.into()))
    }

    pub fn set_frames<S>(&mut self, frames: &[S]) -> SpinnerResult<()>
    where
        S: AsRef<str>,
    {
        let frames: Vec<String> = frames.iter().map(|s| s.as_ref().to_string()).collect();
        self.state.update(UpdateMessage::Frames(frames))
    }

    pub fn set_output_stream<S>(&mut self, stream: S) -> SpinnerResult<()>
    where
        S: Into<SpinnerStream>,
    {
        self.state.update(UpdateMessage::Stream(stream.into()))
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
        let spinner = Spinner::new("Loading ...");
        assert_eq!(spinner.is_running(), false);
        assert_eq!(*spinner.paused.0.lock().unwrap(), false);
    }

    #[test]
    fn test_start_spinner() {
        let mut spinner = Spinner::new("Loading ...");
        let result = spinner.start();
        assert_eq!(result.is_ok(), true);
        assert_eq!(spinner.is_running(), true);
    }

    #[test]
    fn test_on_start_listener() {
        let mut spinner = Spinner::new("Loading ...");
        let listener_called = Arc::new(AtomicBool::new(false));
        let listener_called_clone = listener_called.clone();

        let listener = move || {
            listener_called_clone.store(true, Ordering::SeqCst);
        };

        spinner.on_start(listener);
        spinner.start().unwrap();
        assert!(listener_called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_on_start_listener_not_called() {
        let mut spinner = Spinner::new("Loading ...");
        let listener_called = Arc::new(AtomicBool::new(false));
        let listener_called_clone = listener_called.clone();
        let listener = move || {
            listener_called_clone.store(true, Ordering::SeqCst);
        };
        spinner.on_start(listener);
        assert!(!listener_called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_start_running_spinner() {
        let mut spinner = Spinner::new("Loading ...");
        spinner.running.store(true, Ordering::SeqCst);
        let result = spinner.start();
        assert_eq!(result.is_err(), true);
        assert_eq!(spinner.is_running(), true);
    }

    #[test]
    fn test_stop_spinner() {
        let mut spinner = Spinner::new("Loading ...");
        spinner.start().unwrap();
        let result = spinner.stop();
        assert_eq!(result.is_ok(), true);
        assert_eq!(spinner.is_running(), false);
    }

    #[test]
    fn test_on_stop_listener() {
        let mut spinner = Spinner::new("Loading ...");
        let listener_called = Arc::new(AtomicBool::new(false));
        let listener_called_clone = listener_called.clone();
        let listener = move |duration: Duration| {
            listener_called_clone.store(true, Ordering::SeqCst);
            assert!(duration > Duration::from_secs(0));
        };
        spinner.on_stop(listener);
        spinner.start().unwrap();
        std::thread::sleep(Duration::from_millis(100));
        spinner.stop().unwrap();
        assert!(listener_called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_on_stop_listener_not_called() {
        let mut spinner = Spinner::new("Loading ...");
        let listener_called = Arc::new(AtomicBool::new(false));
        let listener_called_clone = listener_called.clone();
        let listener = move |_: Duration| {
            listener_called_clone.store(true, Ordering::SeqCst);
        };
        spinner.on_stop(listener);
        assert!(!listener_called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_stop_stopped_spinner() {
        let mut spinner = Spinner::new("Loading ...");
        let result = spinner.stop();
        assert_eq!(result.is_err(), true);
        assert_eq!(spinner.is_running(), false);
    }

    #[test]
    fn test_start_stop() {
        let mut spinner = Spinner::new("Loading ...");
        assert_eq!(spinner.start().is_ok(), true);
        assert_eq!(spinner.running.load(Ordering::SeqCst), true);
        assert_eq!(spinner.start().is_err(), true);
        assert_eq!(spinner.stop().is_ok(), true);
        assert_eq!(spinner.is_running(), false);
    }

    #[test]
    fn test_start_multiple_times() {
        let mut spinner = Spinner::new("Loading ...");
        assert!(!spinner.is_running());

        spinner.start().unwrap();
        assert!(spinner.is_running());

        let result = spinner.start();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Spinner is already running"
        );

        spinner.stop().unwrap();
        assert!(!spinner.is_running());
    }

    #[test]
    fn test_stop_multiple_times() {
        let mut spinner = Spinner::new("Loading ...");
        assert!(!spinner.is_running());

        let result = spinner.stop();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Spinner is not running");

        spinner.start().unwrap();
        assert!(spinner.is_running());

        spinner.stop().unwrap();
        assert!(!spinner.is_running());

        let result = spinner.stop();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Spinner is not running");

        spinner.start().unwrap();
        assert!(spinner.is_running());

        spinner.stop().unwrap();
        assert!(!spinner.is_running());
    }

    #[test]
    fn test_pause_spinner() {
        let mut spinner = Spinner::new("Loading ...");
        spinner.start().unwrap();
        assert_eq!(spinner.pause().is_ok(), true);
    }

    #[test]
    fn test_on_pause_listener() {
        let mut spinner = Spinner::new("Loading ...");
        let listener_called = Arc::new(AtomicBool::new(false));
        let listener_called_clone = listener_called.clone();
        let listener = move |duration: Option<Duration>| {
            println!("duration: {:?}", duration);
            listener_called_clone.store(true, Ordering::SeqCst);
            if let Some(duration) = duration {
                assert!(duration > Duration::from_secs(0));
            }
        };
        spinner.start().unwrap();
        spinner.on_pause(listener);
        spinner.pause().unwrap();
        assert!(listener_called.load(Ordering::SeqCst));
    }
    
    #[test]
    fn test_pause_paused_spinner() {
        let mut spinner = Spinner::new("Loading ...");
        spinner.start().unwrap();
        spinner.pause().unwrap();
        assert_eq!(spinner.pause().is_err(), true);
    }

    #[test]
    fn test_pause_stopped_spinner() {
        let mut spinner = Spinner::new("Loading ...");
        assert_eq!(spinner.pause().is_err(), true);
    }

    #[test]
    fn test_resume_spinner() {
        let mut spinner = Spinner::new("Loading ...");
        spinner.start().unwrap();
        spinner.pause().unwrap();
        assert_eq!(spinner.resume().is_ok(), true);
    }

    #[test]
    fn test_resume_running_spinner() {
        let mut spinner = Spinner::new("Loading ...");
        spinner.start().unwrap();
        assert_eq!(spinner.resume().is_err(), true);
    }
    #[test]
    fn test_resume_unpaused_spinner() {
        let mut spinner = Spinner::new("Loading ...");
        spinner.start().unwrap();
        assert_eq!(spinner.resume().is_err(), true);
    }

    #[test]
    fn test_resume_stopped_spinner() {
        let mut spinner = Spinner::new("Loading ...");
        assert_eq!(spinner.resume().is_err(), true);
    }

    #[test]
    fn test_pause_resume_multiple_times() {
        let mut spinner = Spinner::new("Loading ...");
        assert!(spinner.start().is_ok());
        assert!(spinner.pause().is_ok());
        assert!(spinner.resume().is_ok());
        assert!(spinner.pause().is_ok());
        assert!(spinner.resume().is_ok());
        assert!(spinner.stop().is_ok());
    }

    #[test]
    fn test_resume_while_not_paused() {
        let mut spinner = Spinner::new("Loading ...");
        assert!(spinner.start().is_ok());
        assert!(spinner.resume().is_err());
        assert!(spinner.stop().is_ok());
    }

    #[test]
    fn test_drop() {
        let mut spinner = Spinner::new("Loading ...");
        assert!(spinner.start().is_ok());
        drop(spinner);
    }
}
