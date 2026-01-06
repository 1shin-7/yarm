use serde::{Deserialize, Serialize};
use super::resolution::Resolution;
use super::orientation::Orientation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Monitor {
    pub id: String,
    pub name: String,         // Friendly name (e.g. "Dell U2415")
    pub device_name: String,  // OS device name (e.g. "\\.\\DISPLAY1")
    pub current_resolution: Resolution,
    pub current_orientation: Orientation,
    pub position: (i32, i32),
    pub is_primary: bool,
    #[serde(skip)]
    pub available_resolutions: Vec<Resolution>,
}
