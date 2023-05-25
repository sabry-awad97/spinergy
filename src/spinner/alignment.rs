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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_horizontal_padding_left() {
        let alignment = Alignment::Left;
        let terminal_width = 10;
        let text_width = 5;
        let expected_padding = "";

        let padding = alignment.get_horizontal_padding(terminal_width, text_width);

        assert_eq!(padding, expected_padding);
    }

    #[test]
    fn test_get_horizontal_padding_right() {
        let alignment = Alignment::Right;
        let terminal_width = 10;
        let text_width = 5;
        let expected_padding = "     ";

        let padding = alignment.get_horizontal_padding(terminal_width, text_width);

        assert_eq!(padding, expected_padding);
    }

    #[test]
    fn test_get_horizontal_padding_center() {
        let alignment = Alignment::Center;
        let terminal_width = 10;
        let text_width = 5;
        let expected_padding = "  ";

        let padding = alignment.get_horizontal_padding(terminal_width, text_width);

        assert_eq!(padding, expected_padding);
    }

    #[test]
    fn test_from_invalid() {
        let alignment_str = "invalid";
        let alignment = Alignment::from(alignment_str);
        assert!(matches!(alignment, Alignment::Left));
    }

    #[test]
    fn test_from_lowercase() {
        let alignment_str = "center";
        let alignment = Alignment::from(alignment_str);
        assert!(matches!(alignment, Alignment::Center));
    }

    #[test]
    fn test_from_uppercase() {
        let alignment_str = "CENTER";
        let alignment = Alignment::from(alignment_str);
        assert!(matches!(alignment, Alignment::Center));
    }
}
