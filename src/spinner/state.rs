use std::io::Write;
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicBool, Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use super::builtins::{get_spinner_data, SpinnerStyle};
use super::{channel::Channel, message::UpdateMessage};
use crate::{spinner::message::SpinnerMessage, SpinnerError, SpinnerResult, SpinnerStream};

#[derive(Clone)]
pub struct SpinnerState {
    channel: Channel<SpinnerMessage>,
    output: Arc<Mutex<SpinnerStream>>,
    dots: String,
    text: String,
    spinner_style: SpinnerStyle,
    frames: Vec<String>,
    frame_duration: u64,
    reverse: Arc<AtomicBool>,
}

impl SpinnerState {
    pub fn new(message: impl Into<String>) -> Self {
        let channel = Channel::new();

        let stream = SpinnerStream::default();
        let output = Arc::new(Mutex::new(stream));

        let (text, dots) = trim_trailing_dots(message);

        let spinner_style = SpinnerStyle::default();
        let data = get_spinner_data(&spinner_style);
        let frames = data.frames;
        let frame_duration = data.frame_duration;

        let reverse = Arc::new(AtomicBool::new(false));

        Self {
            channel,
            output,
            dots,
            text,
            spinner_style,
            frames,
            frame_duration,
            reverse
        }
    }

    pub fn update(&mut self, message: UpdateMessage) -> SpinnerResult<()> {
        self.channel
            .try_send(SpinnerMessage::Update(Ok(message)))
            .map_err(|_| "Failed to send message through channel".into())
    }

    pub fn stop(&self) -> SpinnerResult<()> {
        self.channel
            .try_send(SpinnerMessage::Stop)
            .map_err(|_| "Failed to send stop message through channel".into())
    }

    pub fn set_reverse(&mut self, reverse: bool) {
        self.reverse.store(reverse, Ordering::SeqCst);
    }

    pub fn spin(
        &mut self,
        running: Arc<AtomicBool>,
        paused: Arc<(Mutex<bool>, Condvar)>,
    ) -> SpinnerResult<()> {
        write!(self.output.lock().unwrap(), "\x1B[?25l")
            .map_err(|e| SpinnerError::new(&e.to_string()))?; // hide cursor

        let mut dot_count = self.dots.len();
        let mut current_index = 0;

        loop {
            if !running.load(Ordering::SeqCst) {
                break;
            }

            let (lock, cvar) = &*paused;
            let mut paused = lock.lock().unwrap();
            while *paused {
                paused = cvar.wait(paused).unwrap();
            }

            let frames = &self.frames;
            let frames_length = frames.len();
            let frame = &frames[current_index % frames_length];
            let dots = ".".repeat(dot_count.min(self.dots.len()));

            self.print_frame(&frame, &dots)?;

            current_index = match self.reverse.load(Ordering::SeqCst) {
                true => (current_index + frames_length - 1) % frames_length,
                false => (current_index + 1) % frames_length,
            };

            dot_count = (dot_count + 1) % (frames_length * 4);

            thread::sleep(Duration::from_millis(self.frame_duration));

            if let Ok(spin_message) = self.channel.try_receive() {
                match spin_message {
                    SpinnerMessage::Stop => {
                        if self.channel.try_send(SpinnerMessage::Stop).is_err() {
                            return Err("Failed to send message through channel".into());
                        }
                        return Err("Spinner stopped".into());
                    }
                    SpinnerMessage::Update(result) => match result {
                        Ok(UpdateMessage::Message(mesage)) => {
                            let (text, dots) = trim_trailing_dots(mesage);
                            self.text = text;
                            self.dots = dots;
                            dot_count = 0;
                        }
                        Ok(UpdateMessage::Style(spinner_style)) => {
                            self.spinner_style = spinner_style;
                            let data = get_spinner_data(&self.spinner_style);
                            self.frames = data.frames;
                            self.frame_duration = data.frame_duration;
                            current_index = 0;
                        }
                        Err(_) => return Err("Failed to receive update message".into()),
                    },
                }
            }
        }
        Ok(())
    }

    fn print_frame(&self, frame: &str, dots: &str) -> SpinnerResult<()> {
        let output_str = format!("{}  {}{}", frame, self.text, dots);
        let mut w = self.output.lock().unwrap();
        write!(w, "\r{}\x1B[K", output_str).map_err(|e| SpinnerError::new(&e.to_string()))?;
        w.flush().map_err(|e| SpinnerError::new(&e.to_string()))
    }
}

fn trim_trailing_dots(message: impl Into<String>) -> (String, String) {
    let mut text = String::new();
    let mut message_dots = String::new();

    let mut found_non_dot = false;

    for c in message.into().chars().rev() {
        if c == '.' && !found_non_dot {
            message_dots.push('.')
        } else {
            found_non_dot = true;
            text.insert(0, c);
        }
    }

    (text, message_dots)
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    use crate::spinner::message::SpinnerMessage;

    #[test]
    fn test_spinner_state_new() {
        let spinner_state = SpinnerState::new("Loading ...");

        // Ensure the initial dot count is correct
        assert_eq!(spinner_state.dots.len(), 3);

        // Ensure the initial text is correct
        assert_eq!(spinner_state.text, "Loading ");

        // Ensure the output stream is initialized
        assert!(matches!(
            *spinner_state.output.lock().unwrap(),
            SpinnerStream::Stdout
        ))
    }

    #[test]
    fn test_spinner_state_update() {
        let mut state = SpinnerState::new("Loading ...");
        // Send an update message
        let update_message = UpdateMessage::Message("Updating...".to_string());
        state.update(update_message).unwrap();
        // Receive the update message
        let spin_message = state.channel.try_receive().unwrap();
        if let SpinnerMessage::Update(result) = spin_message {
            if let Ok(UpdateMessage::Message(message)) = result {
                // Ensure the received message matches the sent message
                assert_eq!(message, "Updating...");
            } else {
                panic!("Expected an UpdateMessage::Message");
            }
        } else {
            panic!("Expected a SpinnerMessage::Update");
        }
    }

    #[test]
    fn test_spinner_state_stop() {
        let spinner_state = SpinnerState::new("Loading ...");

        // Send a stop message
        spinner_state.stop().unwrap();

        // Receive the stop message
        let spin_message = spinner_state.channel.try_receive().unwrap();
        assert!(matches!(spin_message, SpinnerMessage::Stop))
    }

    #[test]
    fn test_spin_thread() {
        let running = Arc::new(AtomicBool::new(true));
        let paused = Arc::new((Mutex::new(false), Condvar::new()));
        let mut state = SpinnerState::new("Loading ...");
        let running_clone = running.clone();
        let paused_clone = paused.clone();
        let spinner_thread = thread::spawn(move || {
            let result = state.spin(running_clone, paused_clone);
            assert!(result.is_ok());
        });
        running.store(false, Ordering::SeqCst);
        spinner_thread.join().unwrap();
    }

    #[test]
    fn test_trim_trailing_dots_mixed_text_and_dots() {
        let input = String::from("Hello... World....");
        let expected = (String::from("Hello... World"), String::from("...."));
        let result = trim_trailing_dots(input);
        assert_eq!(result, expected);
    }
}
