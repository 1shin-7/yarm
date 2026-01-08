use crate::display::Resolution;
use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub general: GeneralConfig,
    #[serde(default)]
    pub profiles: Vec<Profile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_reset_timeout")]
    pub reset_timeout: u8,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            reset_timeout: default_reset_timeout(),
        }
    }
}

fn default_reset_timeout() -> u8 {
    15
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Profile {
    pub name: String,
    pub settings: Vec<MonitorSetting>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MonitorSetting {
    pub monitor_id: String,
    pub resolution: Resolution,
}

pub struct ConfigManager;

impl ConfigManager {
    fn get_config_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "yarm", "yarm")
            .ok_or_else(|| anyhow!("Could not determine config directory"))?;
        let config_dir = proj_dirs.config_dir();

        if !config_dir.exists() {
            fs::create_dir_all(config_dir)?;
        }

        Ok(config_dir.join("config.toml"))
    }

    pub fn load() -> Result<AppConfig> {
        let path = Self::get_config_path()?;
        if !path.exists() {
            return Ok(AppConfig::default());
        }

        let content = fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(config: &AppConfig) -> Result<()> {
        let path = Self::get_config_path()?;
        let content = toml::to_string_pretty(config)?;
        fs::write(path, content)?;
        Ok(())
    }
}
