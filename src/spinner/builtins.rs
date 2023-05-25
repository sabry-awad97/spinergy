use std::collections::HashMap;

use lazy_static::lazy_static;
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

impl<'a> Into<SpinnerStyle> for &'a str {
    fn into(self) -> SpinnerStyle {
        let style = SpinnerStyle::deserialize(&serde_json::to_value(self).unwrap());
        style.expect(&format!("Unsupported spinner style: {}", self))
    }
}

impl Default for SpinnerStyle {
    fn default() -> Self {
        Self::CircleHalves
    }
}

lazy_static! {
    pub static ref SPINNER_COLLECTION: HashMap<SpinnerStyle, SpinnerData> = {
        let frames_str = include_str!("../data/frames.toml");
        let frames: HashMap<String, SpinnerData> = toml::from_str(frames_str).unwrap();
        let mut spinners = HashMap::new();
        for (key, value) in frames {
            let style = SpinnerStyle::deserialize(&serde_json::to_value(&key).unwrap()).unwrap();
            spinners.insert(style, value);
        }
        spinners
    };
}

pub fn get_spinner_data(name: &SpinnerStyle) -> SpinnerData {
    SPINNER_COLLECTION.get(name).cloned().unwrap()
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

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

    #[test]
    fn test_spinner_collection_contains_styles() {
        // Check if all spinner styles are present in the collection
        for style in SpinnerStyle::iter() {
            assert!(SPINNER_COLLECTION.contains_key(&style));
        }
    }

    #[test]
    fn test_spinner_collection_frame_duration() {
        // Check if frame duration is non-zero for all spinner styles
        for (_, spinner_data) in SPINNER_COLLECTION.iter() {
            assert_ne!(spinner_data.frame_duration, 0);
        }
    }

    #[test]
    fn test_spinner_collection_frame_count() {
        // Check if each spinner style has at least one frame
        for (_, spinner_data) in SPINNER_COLLECTION.iter() {
            assert!(!spinner_data.frames.is_empty());
        }
    }

    #[test]
    fn test_spinner_collection_deserialization() {
        // Check if deserialization of all spinner styles is successful
        for style in SpinnerStyle::iter() {
            let spinner_data = &SPINNER_COLLECTION[&style];
            let serialized = serde_json::to_string(spinner_data).unwrap();
            let deserialized: SpinnerData = serde_json::from_str(&serialized).unwrap();
            assert_eq!(deserialized, *spinner_data);
        }
    }

    #[test]
    fn test_spinner_style_conversion() {
        let spinner_style: SpinnerStyle = "CircleHalves".into();
        assert_eq!(spinner_style, SpinnerStyle::default());
    }

    #[test]
    #[should_panic]
    fn test_spinner_style_conversion_invalid() {
        let key = r#"InvalidStyle"#;
        let _: SpinnerStyle = key.into();
    }

    #[test]
    fn test_get_spinner_data_existing_spinner() {
        let spinner_name = SpinnerStyle::CircleHalves;
        let spinner_data = get_spinner_data(&spinner_name);

        // Ensure the returned spinner data matches the expected data
        assert_eq!(spinner_data.frames, ["◐", "◓", "◑", "◒"]);
        assert_eq!(spinner_data.frame_duration, 50);
    }
}
