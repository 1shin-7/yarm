use clap::{Parser, Subcommand};
use anyhow::{anyhow, Result};
use crate::config::ConfigManager;
use crate::display::DisplayManager;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Switch to a specific profile
    Switch {
        /// Name of the profile to switch to
        profile_name: String,
    },
    // Future expansion
    List,
}

pub fn handle_cli(cli: Cli) -> Result<()> {
    match cli.command {
        Some(Commands::Switch { profile_name }) => {
            let config = ConfigManager::load()?;
            let profile = config.profiles.iter()
                .find(|p| p.name == profile_name)
                .ok_or_else(|| anyhow!("Profile '{}' not found", profile_name))?;

            println!("Switching to profile: {}", profile_name);
            
            for setting in &profile.settings {
                println!("Setting monitor {} to {}", setting.monitor_id, setting.resolution);
                if let Err(e) = DisplayManager::set_resolution(&setting.monitor_id, &setting.resolution) {
                    eprintln!("Failed to set resolution for {}: {}", setting.monitor_id, e);
                }
            }
            Ok(())
        }
        Some(Commands::List) => {
            let config = ConfigManager::load()?;
            println!("Available profiles:");
            for profile in config.profiles {
                println!("- {}", profile.name);
            }
            Ok(())
        }
        None => Ok(()), // Should launch GUI
    }
}
