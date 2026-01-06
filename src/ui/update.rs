use crate::display::DisplayManager;
use crate::ui::loader::load_data;
use crate::ui::message::Message;
use crate::ui::state::YarmApp;
use crate::utils::config::{ConfigManager, MonitorSetting, Profile};
use iced::Task;

pub fn update(app: &mut YarmApp, message: Message) -> Task<Message> {
    match message {
        Message::Loaded(Ok((monitors, config))) => {
            app.monitors = monitors.clone();
            app.config = config;

            // Initialize staging with current system state
            app.staging_resolutions.clear();
            for m in &app.monitors {
                app.staging_resolutions
                    .insert(m.id.clone(), m.current_resolution.clone());
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
        Message::ApplyToSystem => {
            let mut errors = Vec::new();
            for (id, res) in &app.staging_resolutions {
                if let Some(monitor) = app.monitors.iter().find(|m| &m.id == id) {
                    if let Err(e) = DisplayManager::set_resolution(&monitor.device_name, res) {
                        errors.push(format!("{}: {}", monitor.name, e));
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
                settings.push(MonitorSetting {
                    monitor_id: id.clone(),
                    resolution: res.clone(),
                });
            }

            let new_profile = Profile {
                name: app.new_profile_name.clone(),
                settings,
            };

            app.config
                .profiles
                .retain(|p| p.name != app.new_profile_name);
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
                        app.staging_resolutions
                            .insert(setting.monitor_id.clone(), setting.resolution.clone());
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
        Message::Refresh => Task::perform(load_data(), Message::Loaded),
        Message::WindowResized(size) => {
            if app.debug {
                println!("Window resized to: {}x{}", size.width, size.height);
            }
            Task::none()
        }
    }
}
