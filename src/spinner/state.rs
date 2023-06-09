use std::io::Write;
use std::sync::atomic::Ordering;
use std::sync::{atomic::AtomicBool, Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use colored::*;
use unicode_width::UnicodeWidthStr;

use super::alignment::Alignment;
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
    alignment: Alignment,
    style_color: Option<Color>,
    text_color: Option<Color>,
    dot_color: Option<Color>,
    interval: Option<Duration>,
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
        let alignment = Alignment::default();

        let style_color = Some(Color::Magenta);
        let text_color = None;
        let dot_color = Some(Color::Magenta);

        let interval = None;

        Self {
            channel,
            output,
            dots,
            text,
            spinner_style,
            frames,
            frame_duration,
            reverse,
            alignment,
            style_color,
            text_color,
            dot_color,
            interval,
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

            self.print(frame, &self.text, &dots)?;

            current_index = match self.reverse.load(Ordering::SeqCst) {
                true => (current_index + frames_length - 1) % frames_length,
                false => (current_index + 1) % frames_length,
            };

            dot_count = (dot_count + 1) % (frames_length * 4);

            thread::sleep(
                self.interval
                    .unwrap_or(Duration::from_millis(self.frame_duration)),
            );

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
                        Ok(UpdateMessage::Alignment(alignment)) => {
                            self.alignment = alignment;
                        }
                        Ok(UpdateMessage::Colors(style_color, text_color, dot_color)) => {
                            self.style_color = style_color;
                            self.text_color = text_color;
                            self.dot_color = dot_color;
                        }
                        Ok(UpdateMessage::FramesPerSecond(fps)) => {
                            let frame_duration = 1.0 / fps;
                            let duration = Duration::from_secs_f64(frame_duration);
                            self.interval = Some(duration);
                        }
                        Ok(UpdateMessage::Speed(rpm)) => {
                            const SECONDS_PER_MINUTE: f64 = 60.0;
                            let duration = Duration::from_secs_f64(SECONDS_PER_MINUTE / rpm)
                                / self.frames.len() as u32;
                            self.interval = Some(duration);
                        }
                        Ok(UpdateMessage::Frames(frames)) => {
                            self.frames = frames;
                            current_index = 0;
                            dot_count = 0;
                        }
                        Ok(UpdateMessage::Stream(output)) => {
                            self.output = Arc::new(Mutex::new(output));
                        }
                        Err(_) => return Err("Failed to receive update message".into()),
                    },
                }
            }
        }
        Ok(())
    }

    fn print(&self, frame: &str, text: &str, dots: &str) -> SpinnerResult<()> {
        let (width, _) = get_terminal_size();
        let padding_str = self.alignment.get_horizontal_padding(
            width - 2,
            frame.width() + self.text.width() + self.dots.width(),
        );

        let colored_frame = match self.style_color {
            Some(color) => frame.color(color).to_string(),
            None => frame.to_owned(),
        };

        let colored_text = match self.text_color {
            Some(color) => text.color(color).to_string(),
            None => text.to_owned(),
        };

        let colored_dots = match self.dot_color {
            Some(color) => dots.color(color).to_string(),
            None => dots.to_owned(),
        };

        let output_str = format!(
            "{}{} {}{}",
            padding_str, colored_frame, colored_text, colored_dots
        );

        let mut w = self.output.lock().unwrap();
        let clear_line = "\r\x1B[K";
        write!(w, "{}{}", clear_line, output_str).map_err(|e| SpinnerError::new(&e.to_string()))?;
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

fn get_terminal_size() -> (usize, usize) {
    term_size::dimensions().unwrap_or((80, 24))
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
    fn test_update_spinner_state_with_message() {
        let mut state = SpinnerState::new("Loading ...");
        // Send an update message
        let update_message = UpdateMessage::Message("Updating...".to_string());
        let result = state.update(update_message);
        assert!(result.is_ok());
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
        let result = spinner_state.stop();
        assert!(result.is_ok());

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
    fn test_set_reverse_spinner_state() {
        let mut state = SpinnerState::new("Loading");
        state.set_reverse(true);
        assert_eq!(state.reverse.load(Ordering::SeqCst), true);
        // Make assertions for setting reverse to false as well
    }

    #[test]
    fn test_print_spinner_state() {
        let state = SpinnerState::new("Loading");
        let result = state.print("|", "Text", "...");
        assert!(result.is_ok());
    }

    #[test]
    fn test_trim_trailing_dots_mixed_text_and_dots() {
        let input = String::from("Hello... World....");
        let expected = (String::from("Hello... World"), String::from("...."));
        let result = trim_trailing_dots(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_terminal_size() {
        let (width, height) = get_terminal_size();
        assert!(width > 0);
        assert!(height > 0);
    }
}
