use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct SpinnerData {
    pub frames: Vec<String>,
    pub frame_duration: u64,
}
