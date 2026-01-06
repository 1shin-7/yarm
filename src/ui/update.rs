use iced::Task;
use crate::ui::state::YarmApp;
use crate::ui::message::Message;
use crate::ui::loader::load_data;
use crate::display::DisplayManager;
use crate::utils::config::{ConfigManager, MonitorSetting, Profile};

pub fn update(app: &mut YarmApp, message: Message) -> Task<Message> {
    match message {
        Message::Loaded(Ok((monitors, config))) => {
            app.monitors = monitors.clone();
            app.config = config;
            
            // Initialize staging with current system state
            app.staging_resolutions.clear();
            app.staging_orientations.clear();
            for m in &app.monitors {
                app.staging_resolutions.insert(m.id.clone(), m.current_resolution.clone());
                app.staging_orientations.insert(m.id.clone(), m.current_orientation);
            }
            
            app.status_message = "Ready".to_string();
            Task::none()
        }
        Message::Loaded(Err(e)) => {
            app.status_message = format!("Error loading: {}", e);
            Task::none()
        }
        Message::ResolutionChanged(id, res) => {
            app.staging_resolutions.insert(id, res);
            Task::none()
        }
        Message::OrientationChanged(id, orient) => {
            app.staging_orientations.insert(id, orient);
            Task::none()
        }
        Message::ApplyToSystem => {
            let mut errors = Vec::new();
            
            // Apply Resolutions
            for (id, res) in &app.staging_resolutions {
                if let Some(monitor) = app.monitors.iter().find(|m| &m.id == id) {
                     if let Err(e) = DisplayManager::set_resolution(&monitor.device_name, res) {
                        errors.push(format!("Res {}: {}", monitor.name, e));
                     }
                }
            }

            // Apply Orientations
            for (id, orient) in &app.staging_orientations {
                if let Some(monitor) = app.monitors.iter().find(|m| &m.id == id) {
                     // Only set if different? The set_orientation function does basic checks, but cheap to check here too or just force it.
                     // The logic says "highlight ... when pressed other ... apply config".
                     // So we apply staging.
                     if let Err(e) = DisplayManager::set_orientation(&monitor.device_name, *orient) {
                        errors.push(format!("Orient {}: {}", monitor.name, e));
                     }
                }
            }

            if errors.is_empty() {
                app.status_message = "Applied successfully".to_string();
            } else {
                app.status_message = format!("Errors: {}", errors.join("; "));
            }
            Task::perform(load_data(), Message::Loaded)
        }
        Message::OpenSaveDialog => {
            app.show_save_dialog = true;
            app.new_profile_name.clear();
            Task::none()
        }
        Message::CloseSaveDialog => {
            app.show_save_dialog = false;
            Task::none()
        }
        Message::ConfirmSaveProfile => {
            if app.new_profile_name.trim().is_empty() {
                return Task::none();
            }

            let mut settings = Vec::new();
            for (id, res) in &app.staging_resolutions {
                // We should probably save orientation in profile too? 
                // The current Profile struct only has Resolution.
                // I need to update MonitorSetting to include orientation (optional or required).
                // For now, let's assume profiles only save resolution as per original design, 
                // OR I should update Config to save Orientation.
                // Given the requirement "switch ... applied", it implies orientation is part of configuration.
                // But `MonitorSetting` in `config.rs` only has `resolution`.
                // I will skip saving orientation to profile for this step to keep it minimal unless requested, 
                // but the user might expect it. 
                // The prompt was about "creating button", not updating profile system fully.
                // But logically it makes sense. I'll stick to resolution for profiles for now to avoid breaking changes or extra scope.
                settings.push(MonitorSetting {
                    monitor_id: id.clone(),
                    resolution: res.clone(),
                });
            }

            let new_profile = Profile {
                name: app.new_profile_name.clone(),
                settings,
            };

            app.config.profiles.retain(|p| p.name != app.new_profile_name);
            app.config.profiles.push(new_profile);

            if let Err(e) = ConfigManager::save(&app.config) {
                 app.status_message = format!("Failed to save config: {}", e);
            } else {
                 app.status_message = format!("Profile '{}' saved", app.new_profile_name);
            }
            app.show_save_dialog = false;
            Task::none()
        }
        Message::LoadProfile(name) => {
             if let Some(profile) = app.config.profiles.iter().find(|p| p.name == name) {
                 for setting in &profile.settings {
                     if app.monitors.iter().any(|m| m.id == setting.monitor_id) {
                         app.staging_resolutions.insert(setting.monitor_id.clone(), setting.resolution.clone());
                     }
                 }
                 app.status_message = format!("Loaded profile '{}' (click Apply to set)", name);
             }
             Task::none()
        }
        Message::NewProfileNameChanged(name) => {
            app.new_profile_name = name;
            Task::none()
        }
        Message::Refresh => {
             Task::perform(load_data(), Message::Loaded)
        }
        Message::WindowResized(size) => {
            if app.debug {
                println!("Window resized to: {}x{}", size.width, size.height);
            }
            Task::none()
        }
    }
}