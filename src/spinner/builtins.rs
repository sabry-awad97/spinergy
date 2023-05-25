use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SpinnerData {
    pub frames: Vec<String>,
    pub frame_duration: u64,
}

#[derive(Debug, EnumIter, Display, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum SpinnerStyle {
    Aesthetic,
    Arc,
    Arrow,
    Arrow2,
    Arrow3,
    Balloon,
    Balloon2,
    BetaWave,
    BluePulse,
    Bounce,
    BouncingBall,
    BouncingBar,
    BoxBounce,
    BoxBounce2,
    Christmas,
    Circle,
    CircleHalves,
    CircleQuarters,
    Clock,
    Dots,
    Dots10,
    Dots11,
    Dots12,
    Dots13,
    Dots2,
    Dots3,
    Dots4,
    Dots5,
    Dots6,
    Dots7,
    Dots8,
    Dots8Bit,
    Dots9,
    Dqpb,
    Earth,
    FingerDance,
    FistBump,
    Flip,
    Grenade,
    GrowHorizontal,
    GrowVertical,
    Hamburger,
    Hearts,
    Layer,
    Line,
    Line2,
    Material,
    Mindblown,
    Monkey,
    Moon,
    Noise,
    OrangeBluePulse,
    OrangePulse,
    Pipe,
    Point,
    Pong,
    Runner,
    Sand,
    Shark,
    SimpleDots,
    SimpleDotsScrolling,
    Smiley,
    SoccerHeader,
    Speaker,
    SquareCorners,
    Squish,
    Star,
    Star2,
    TimeTravel,
    Toggle,
    Toggle10,
    Toggle11,
    Toggle12,
    Toggle13,
    Toggle2,
    Toggle3,
    Toggle4,
    Toggle5,
    Toggle6,
    Toggle7,
    Toggle8,
    Toggle9,
    Triangle,
    Weather,
}

impl Default for SpinnerStyle {
    fn default() -> Self {
        Self::CircleHalves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_style_equality() {
        assert_eq!(SpinnerStyle::CircleHalves, SpinnerStyle::CircleHalves);
        // Add more equality assertions for other spinner styles
    }

    #[test]
    fn test_spinner_style_display() {
        assert_eq!(SpinnerStyle::CircleHalves.to_string(), "CircleHalves");
        // Add more assertions for other spinner styles' display names
    }

    #[test]
    fn test_spinner_style_clone() {
        let style = SpinnerStyle::Dots;
        let cloned_style = style.clone();
        assert_eq!(style, cloned_style);
    }

    #[test]
    fn test_spinner_style_deserialize() {
        // Simulating deserialization from JSON or other formats
        let json = r#""CircleHalves""#;
        let deserialized: SpinnerStyle = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, SpinnerStyle::CircleHalves);
    }

    #[test]
    fn test_spinner_style_serialize() {
        // Simulating serialization to JSON or other formats
        let style = SpinnerStyle::CircleHalves;
        let serialized = serde_json::to_string(&style).unwrap();
        assert_eq!(serialized, r#""CircleHalves""#);
    }

    #[test]
    fn test_default_spinner_style() {
        let default_style = SpinnerStyle::default();
        assert_eq!(default_style, SpinnerStyle::CircleHalves);
    }
}
