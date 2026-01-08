use crate::display::{Monitor, Orientation, Resolution};
use crate::utils::config::AppConfig;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<(Vec<Monitor>, AppConfig), String>),
    ResolutionChanged(String, Resolution),
    OrientationChanged(String, Orientation),
    ApplyToSystem,
    OpenSaveDialog,
    CloseSaveDialog,
    ConfirmSaveProfile,
    LoadProfile(String),
    NewProfileNameChanged(String),
    Refresh,
    WindowResized(iced::Size),
    // Confirmation Timer
    Tick,
    ConfirmResolution,
    RevertResolution,
    // Profile Deletion
    RequestDeleteProfile(String),
    ConfirmDeleteProfile,
    CancelDeleteProfile,
}

pub struct YarmApp {
    pub monitors: Vec<Monitor>,
    pub config: AppConfig,
    pub staging_resolutions: HashMap<String, Resolution>,
    pub staging_orientations: HashMap<String, Orientation>,
    pub new_profile_name: String,
    pub status_message: String,
    pub show_save_dialog: bool,
    pub debug: bool,
    // Confirmation state
    pub waiting_for_confirmation: bool,
    pub confirmation_timer: u8,
    pub backup_resolutions: HashMap<String, Resolution>,
    // Profile Deletion
    pub profile_to_delete: Option<String>,
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
            waiting_for_confirmation: false,
            confirmation_timer: 0,
            backup_resolutions: HashMap::new(),
            profile_to_delete: None,
        }
    }
}
