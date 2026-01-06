use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
    pub frequency: u32,
    pub bits_per_pixel: u32,
}

impl std::fmt::Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x{} @ {}Hz ({}bit)",
            self.width, self.height, self.frequency, self.bits_per_pixel
        )
    }
}
