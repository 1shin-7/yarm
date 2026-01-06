use crate::utils::config::AppConfig;
use crate::display::{Monitor, Resolution, Orientation};
use std::collections::HashMap;

pub struct YarmApp {
    pub monitors: Vec<Monitor>,
    pub config: AppConfig,
    pub staging_resolutions: HashMap<String, Resolution>,
    pub staging_orientations: HashMap<String, Orientation>,
    pub new_profile_name: String,
    pub status_message: String,
    pub show_save_dialog: bool,
    pub debug: bool,
}

impl Default for YarmApp {
    fn default() -> Self {
        Self {
            monitors: Vec::new(),
            config: AppConfig::default(),
            staging_resolutions: HashMap::new(),
            staging_orientations: HashMap::new(),
            new_profile_name: String::new(),
            status_message: "Loading...".to_string(),
            show_save_dialog: false,
            debug: false,
        }
    }
}
