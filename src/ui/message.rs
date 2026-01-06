use crate::display::{Monitor, Resolution};
use crate::utils::config::AppConfig;

#[derive(Debug, Clone)]
pub enum Message {
    Loaded(Result<(Vec<Monitor>, AppConfig), String>),
    ResolutionChanged(String, Resolution),
    ApplyToSystem,
    OpenSaveDialog,
    CloseSaveDialog,
    ConfirmSaveProfile,
    LoadProfile(String),
    NewProfileNameChanged(String),
    Refresh,
    WindowResized(iced::Size),
}
