# Spinergy

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/example/spinergy/blob/main/LICENSE)

Spinergy is a Rust library that provides a simple and customizable spinner for displaying progress or waiting indicators in command-line interfaces.

## Features

- Display a spinner with customizable styles and colors.
- Pause and resume the spinner.
- Set the message, style, color scheme, alignment, frames, speed, and output stream of the spinner.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
spinergy = { git = "https://github.com/sabry-awad97/spinergy" }
```

## Example

```rs
use spinergy::Spinner;

fn main() {
    let mut spinner = Spinner::new("Processing...");

    spinner.start().unwrap();

    spinner.stop().unwrap();
}
```

## API

The following are the main methods provided by the `Spinner` struct:

- `new(message: impl Into<String>) -> Spinner`: Constructs a new `Spinner` with the specified message.
- `start() -> SpinnerResult<()>`: Starts the spinner.
- `stop() -> SpinnerResult<()>`: Stops the spinner.
- `pause() -> SpinnerResult<()>`: Pauses the spinner.
- `resume() -> SpinnerResult<()>`: Resumes the spinner.
- `is_running() -> bool`: Checks if the spinner is running.
- `set_message<T>(&mut self, message: T) -> SpinnerResult<()>`: Sets the message of the spinner.
- `set_style(&mut self, style: impl Into<SpinnerStyle>) -> SpinnerResult<()>`: Sets the style of the spinner.
- `set_color_scheme<U>(&mut self, style_color: U, message_color: U, dots_color: U) -> SpinnerResult<()>`: Sets the color scheme of the spinner.
- `set_reverse(&mut self, reverse: bool) -> SpinnerResult<()>`: Sets the spinning direction of the spinner.
- `set_alignment<T>(&mut self, alignment: T) -> SpinnerResult<()>`: Sets the alignment of the spinner.
- `set_fps<V>(&mut self, fps: V) -> SpinnerResult<()>`: Sets the frames per second of the spinner.
- `set_speed<V>(&mut self, rpm: V) -> SpinnerResult<()>`: Sets the speed (rotations per minute) of the spinner.
- `set_frames<S>(&mut self, frames: &[S]) -> SpinnerResult<()>`: Sets the frames of the spinner.
- `set_output_stream<S>(&mut self, stream: S) -> SpinnerResult<()>`: Sets the output stream of the spinner.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/sabry-awad97/spinergy/blob/main/LICENSE) file for details.
