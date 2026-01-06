use crate::display::{DisplayManager, Monitor};
use crate::utils::config::{AppConfig, ConfigManager};

pub async fn load_data() -> Result<(Vec<Monitor>, AppConfig), String> {
    let monitors = DisplayManager::enumerate_monitors().map_err(|e| e.to_string())?;
    let config = ConfigManager::load().map_err(|e| e.to_string())?;
    Ok((monitors, config))
}
