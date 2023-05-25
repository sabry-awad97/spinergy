#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Alignment {
    pub fn get_horizontal_padding(&self, terminal_width: usize, text_width: usize) -> String {
        let padding = match self {
            Alignment::Left => 0,
            Alignment::Center => (terminal_width - text_width) / 2,
            Alignment::Right => terminal_width - text_width,
        };

        " ".repeat(padding)
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Left
    }
}

impl From<&str> for Alignment {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "center" => Self::Center,
            "right" => Self::Right,
            _ => Self::Left,
        }
    }
}

impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "left"),
            Self::Center => write!(f, "center"),
            Self::Right => write!(f, "right"),
        }
    }
}
