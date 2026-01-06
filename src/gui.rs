use iced::widget::{button, column, container, row, scrollable, text, pick_list, text_input};
use iced::{Alignment, Element, Length, Task, Theme};
use crate::config::{AppConfig, ConfigManager, MonitorSetting, Profile};
use crate::display::{DisplayManager, Monitor, Resolution};

pub fn run() -> iced::Result {
    iced::application("Yarm - Yet Another Resolution Manager", YarmApp::update, YarmApp::view)
        .theme(YarmApp::theme)
        .run_with(YarmApp::new)
}

struct YarmApp {
    monitors: Vec<Monitor>,
    config: AppConfig,
    staging_resolutions: std::collections::HashMap<String, Resolution>,
    new_profile_name: String,
    status_message: String,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<(Vec<Monitor>, AppConfig), String>),
    ResolutionChanged(String, Resolution),
    ApplyToSystem,
    SaveProfile,
    DeleteProfile(String),
    LoadProfile(String),
    NewProfileNameChanged(String),
    Refresh,
}

impl Default for YarmApp {
    fn default() -> Self {
        Self {
            monitors: Vec::new(),
            config: AppConfig::default(),
            staging_resolutions: std::collections::HashMap::new(),
            new_profile_name: String::new(),
            status_message: "Loading...".to_string(),
        }
    }
}

impl YarmApp {
    fn new() -> (Self, Task<Message>) {
        (
            Self::default(),
            Task::perform(load_data(), Message::Loaded),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded(Ok((monitors, config))) => {
                self.monitors = monitors.clone();
                self.config = config;
                
                // Initialize staging with current system state
                self.staging_resolutions.clear();
                for m in &self.monitors {
                    self.staging_resolutions.insert(m.id.clone(), m.current_resolution.clone());
                }
                
                self.status_message = "Ready".to_string();
                Task::none()
            }
            Message::Loaded(Err(e)) => {
                self.status_message = format!("Error loading: {}", e);
                Task::none()
            }
            Message::ResolutionChanged(id, res) => {
                self.staging_resolutions.insert(id, res);
                Task::none()
            }
            Message::ApplyToSystem => {
                let mut errors = Vec::new();
                for (id, res) in &self.staging_resolutions {
                    // Find the device name for this ID
                    if let Some(monitor) = self.monitors.iter().find(|m| &m.id == id) {
                         if let Err(e) = DisplayManager::set_resolution(&monitor.device_name, res) {
                            errors.push(format!("{}: {}", monitor.name, e));
                         }
                    }
                }

                if errors.is_empty() {
                    self.status_message = "Applied successfully".to_string();
                } else {
                    self.status_message = format!("Errors: {}", errors.join("; "));
                }
                
                // Reload monitor state to confirm
                Task::perform(load_data(), Message::Loaded)
            }
            Message::SaveProfile => {
                if self.new_profile_name.trim().is_empty() {
                    self.status_message = "Please enter a profile name".to_string();
                    return Task::none();
                }

                let mut settings = Vec::new();
                for (id, res) in &self.staging_resolutions {
                    settings.push(MonitorSetting {
                        monitor_id: id.clone(),
                        resolution: res.clone(),
                    });
                }

                let new_profile = Profile {
                    name: self.new_profile_name.clone(),
                    settings,
                };

                // Remove existing if any
                self.config.profiles.retain(|p| p.name != self.new_profile_name);
                self.config.profiles.push(new_profile);

                if let Err(e) = ConfigManager::save(&self.config) {
                     self.status_message = format!("Failed to save config: {}", e);
                } else {
                     self.status_message = format!("Profile '{}' saved", self.new_profile_name);
                     self.new_profile_name.clear();
                }
                Task::none()
            }
            Message::DeleteProfile(name) => {
                self.config.profiles.retain(|p| p.name != name);
                if let Err(e) = ConfigManager::save(&self.config) {
                     self.status_message = format!("Failed to save config: {}", e);
                } else {
                     self.status_message = format!("Profile '{}' deleted", name);
                }
                Task::none()
            }
            Message::LoadProfile(name) => {
                 if let Some(profile) = self.config.profiles.iter().find(|p| p.name == name) {
                     for setting in &profile.settings {
                         // Check if this monitor still exists
                         if self.monitors.iter().any(|m| m.id == setting.monitor_id) {
                             self.staging_resolutions.insert(setting.monitor_id.clone(), setting.resolution.clone());
                         }
                     }
                     self.status_message = format!("Loaded profile '{}' (click Apply to set)", name);
                 }
                 Task::none()
            }
            Message::NewProfileNameChanged(name) => {
                self.new_profile_name = name;
                Task::none()
            }
            Message::Refresh => {
                 Task::perform(load_data(), Message::Loaded)
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Sidebar: Profiles
        let profiles_list = self.config.profiles.iter().fold(column![].spacing(10), |col, profile| {
            col.push(
                row![
                    button(text(&profile.name))
                        .on_press(Message::LoadProfile(profile.name.clone()))
                        .width(Length::Fill),
                    button(text("x"))
                        .on_press(Message::DeleteProfile(profile.name.clone()))
                        .style(button::danger),
                ]
                .spacing(5)
            )
        });

        let sidebar = container(
            column![
                text("Profiles").size(24),
                scrollable(profiles_list).height(Length::Fill),
                text_input("New Profile Name", &self.new_profile_name)
                    .on_input(Message::NewProfileNameChanged),
                button("Save Current as Profile").on_press(Message::SaveProfile).width(Length::Fill),
            ]
            .spacing(20)
        )
        .width(Length::Fixed(250.0))
        .padding(20)
        .style(|_theme| container::Style {
            border: iced::Border { width: 1.0, color: iced::Color::from_rgb(0.8, 0.8, 0.8), radius: 0.0.into() },
            ..Default::default()
        });

        // Main Area: Monitors
        let monitors_list = self.monitors.iter().fold(column![].spacing(20), |col, monitor| {
            let current_staging = self.staging_resolutions.get(&monitor.id).unwrap_or(&monitor.current_resolution);
            
            // Find the selected one in available list to make pick_list happy (equality check)
            let selected_option = monitor.available_resolutions.iter().find(|r| *r == current_staging).cloned();

            let control = pick_list(
                &monitor.available_resolutions[..],
                selected_option,
                |res| Message::ResolutionChanged(monitor.id.clone(), res)
            )
            .width(Length::Fill);

            col.push(
                container(
                    column![
                        text(&monitor.name).size(20).font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }),
                        text(format!("System: {}", monitor.current_resolution)).size(12).color(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                        row![
                            text("Target: ").width(Length::Fixed(60.0)),
                            control
                        ].align_y(Alignment::Center).spacing(10)
                    ]
                    .spacing(10)
                )
                .padding(15)
                .style(|_theme| container::Style {
                    background: Some(iced::Color::from_rgb(0.95, 0.95, 0.95).into()),
                    border: iced::Border { width: 1.0, color: iced::Color::from_rgb(0.9, 0.9, 0.9), radius: 5.0.into() },
                    ..Default::default()
                })
            )
        });

        let main_content = container(
            column![
                text("Monitors").size(24),
                scrollable(monitors_list).height(Length::Fill),
                row![
                    button("Refresh Hardware").on_press(Message::Refresh),
                    button("Apply Changes to System")
                        .on_press(Message::ApplyToSystem)
                        .style(button::primary)
                ]
                .spacing(20)
                .align_y(Alignment::Center),
                text(&self.status_message).size(14)
            ]
            .spacing(20)
        )
        .padding(20)
        .width(Length::Fill);

        row![sidebar, main_content].into()
    }
    
    fn theme(&self) -> Theme {
        Theme::Light
    }
}

async fn load_data() -> Result<(Vec<Monitor>, AppConfig), String> {
    let monitors = DisplayManager::enumerate_monitors().map_err(|e| e.to_string())?;
    let config = ConfigManager::load().map_err(|e| e.to_string())?;
    Ok((monitors, config))
}
