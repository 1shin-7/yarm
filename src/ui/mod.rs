pub mod model;
pub mod theme;
pub mod views;
pub mod widgets;

use crate::display::{DisplayManager, Monitor};
use crate::utils::config::{ConfigManager, MonitorSetting, Profile};
use iced::widget::{container, row};
use iced::{event, Background, Element, Length, Subscription, Task, Theme};

use self::model::{Message, YarmApp};
use self::theme::*;
use crate::utils::config::AppConfig;

pub fn run(debug: bool) -> iced::Result {
    iced::application(
        "Yarm - Yet Another Resolution Manager",
        YarmApp::update,
        YarmApp::view,
    )
    .theme(YarmApp::theme)
    .window(iced::window::Settings {
        size: iced::Size::new(720.0, 640.0), // Updated size
        ..Default::default()
    })
    .subscription(YarmApp::subscription)
    .run_with(move || YarmApp::new(debug))
}

impl YarmApp {
    fn new(debug: bool) -> (Self, Task<Message>) {
        (
            Self {
                debug,
                ..Self::default()
            },
            Task::perform(load_data(), Message::Loaded),
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.debug {
            event::listen_with(|event, _status, _window_id| {
                if let iced::Event::Window(iced::window::Event::Resized(size)) = event {
                    Some(Message::WindowResized(size))
                } else {
                    None
                }
            })
        } else {
            Subscription::none()
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Loaded(Ok((monitors, config))) => {
                self.monitors = monitors.clone();
                self.config = config;

                // Initialize staging with current system state
                self.staging_resolutions.clear();
                self.staging_orientations.clear();
                for m in &self.monitors {
                    self.staging_resolutions
                        .insert(m.id.clone(), m.current_resolution.clone());
                    self.staging_orientations
                        .insert(m.id.clone(), m.current_orientation);
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
            Message::OrientationChanged(id, orient) => {
                self.staging_orientations.insert(id, orient);
                Task::none()
            }
            Message::ApplyToSystem => {
                let mut errors = Vec::new();

                // Apply Resolutions
                for (id, res) in &self.staging_resolutions {
                    if let Some(monitor) = self.monitors.iter().find(|m| &m.id == id) {
                        if let Err(e) = DisplayManager::set_resolution(&monitor.device_name, res) {
                            errors.push(format!("Res {}: {}", monitor.name, e));
                        }
                    }
                }

                // Apply Orientations
                for (id, orient) in &self.staging_orientations {
                    if let Some(monitor) = self.monitors.iter().find(|m| &m.id == id) {
                        if let Err(e) =
                            DisplayManager::set_orientation(&monitor.device_name, *orient)
                        {
                            errors.push(format!("Orient {}: {}", monitor.name, e));
                        }
                    }
                }

                if errors.is_empty() {
                    self.status_message = "Applied successfully".to_string();
                } else {
                    self.status_message = format!("Errors: {}", errors.join("; "));
                }
                Task::perform(load_data(), Message::Loaded)
            }
            Message::OpenSaveDialog => {
                self.show_save_dialog = true;
                self.new_profile_name.clear();
                Task::none()
            }
            Message::CloseSaveDialog => {
                self.show_save_dialog = false;
                Task::none()
            }
            Message::ConfirmSaveProfile => {
                if self.new_profile_name.trim().is_empty() {
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

                self.config
                    .profiles
                    .retain(|p| p.name != self.new_profile_name);
                self.config.profiles.push(new_profile);

                if let Err(e) = ConfigManager::save(&self.config) {
                    self.status_message = format!("Failed to save config: {}", e);
                } else {
                    self.status_message = format!("Profile '{}' saved", self.new_profile_name);
                }
                self.show_save_dialog = false;
                Task::none()
            }
            Message::LoadProfile(name) => {
                if let Some(profile) = self.config.profiles.iter().find(|p| p.name == name) {
                    for setting in &profile.settings {
                        if self.monitors.iter().any(|m| m.id == setting.monitor_id) {
                            self.staging_resolutions
                                .insert(setting.monitor_id.clone(), setting.resolution.clone());
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
            Message::Refresh => Task::perform(load_data(), Message::Loaded),
            Message::WindowResized(size) => {
                if self.debug {
                    println!("Window resized to: {}x{}", size.width, size.height);
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let profiles_section = views::profile::view(&self.config.profiles);
        let main_area = views::monitor::view(
            &self.monitors,
            &self.staging_resolutions,
            &self.staging_orientations,
            &self.status_message,
        );

        let content = container(row![profiles_section, main_area].spacing(30))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(30) // Outer padding for floating feel
            .style(|_theme| container::Style {
                background: Some(Background::Color(COL_BACKGROUND)),
                ..Default::default()
            });

        widgets::dialog::view(
            self.show_save_dialog,
            &self.new_profile_name,
            &self.staging_resolutions,
            content.into(),
        )
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
