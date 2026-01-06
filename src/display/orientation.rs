use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Orientation {
    #[default]
    Landscape = 0,
    Portrait = 1,
    LandscapeFlipped = 2,
    PortraitFlipped = 3,
}

impl Orientation {
    pub fn from_u32(val: u32) -> Self {
        match val {
            0 => Orientation::Landscape,
            1 => Orientation::Portrait,
            2 => Orientation::LandscapeFlipped,
            3 => Orientation::PortraitFlipped,
            _ => Orientation::Landscape,
        }
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }

    pub fn degrees(self) -> &'static str {
        match self {
            Orientation::Landscape => "0°",
            Orientation::Portrait => "90°",
            Orientation::LandscapeFlipped => "180°",
            Orientation::PortraitFlipped => "270°",
        }
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.degrees())
    }
}
