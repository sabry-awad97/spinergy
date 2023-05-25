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
