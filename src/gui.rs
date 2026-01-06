use iced::widget::{button, column, container, row, scrollable, text, pick_list, text_input, stack, horizontal_space, vertical_space};
use iced::{Alignment, Element, Length, Task, Theme, Color, Shadow, Vector, Background};
use iced::border::Radius;
use crate::config::{AppConfig, ConfigManager, MonitorSetting, Profile};
use crate::display::{DisplayManager, Monitor, Resolution};

// --- Theme Colors (Sea Salt Blue Palette) ---

const COL_BACKGROUND: Color = Color::from_rgb(0.96, 0.97, 0.98); // Very light gray-blue

const COL_SURFACE: Color = Color::from_rgb(0.98, 0.98, 0.98); // Almost white/light gray

const COL_PRIMARY: Color = Color::from_rgb(0.36, 0.61, 0.61); // Sea Salt Blue / Muted Teal

const COL_PRIMARY_TEXT: Color = Color::from_rgb(1.0, 1.0, 1.0);

const COL_TEXT_DARK: Color = Color::from_rgb(0.2, 0.25, 0.3);

const COL_TEXT_MUTED: Color = Color::from_rgb(0.5, 0.55, 0.6);



// --- Style Helpers ---

fn card_style(_theme: &Theme) -> container::Style {

    container::Style {

        background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))), // Light gray

        border: iced::Border {

            width: 0.0,

            color: Color::TRANSPARENT,

            radius: Radius::from(12.0),

        },

        shadow: Shadow {

            color: Color::from_rgba(0.0, 0.0, 0.0, 0.03),

            offset: Vector::new(0.0, 2.0),

            blur_radius: 8.0,

        },

                ..Default::default()

            }

        }

        

        fn floating_column_style(_theme: &Theme) -> container::Style {

            container::Style {

                background: Some(Background::Color(COL_SURFACE)), // Simulate glass/floating

                border: iced::Border {

                    width: 1.0,

                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.6), // Subtle highlight border

                    radius: Radius::from(12.0),

                },

                shadow: Shadow {

                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),

                    offset: Vector::new(0.0, 8.0), // Higher float

                    blur_radius: 16.0,

                },

                ..Default::default()

            }

        }

        

        fn primary_button_style(_theme: &Theme, status: button::Status) -> button::Style {

            let base = button::Style {

                background: Some(Background::Color(COL_PRIMARY)),

                text_color: COL_PRIMARY_TEXT,

                border: iced::Border {

                    radius: Radius::from(12.0),

                    ..Default::default()

                },

                ..Default::default()

            };

            match status {

                button::Status::Hovered => button::Style {

                    background: Some(Background::Color(Color { a: 0.9, ..COL_PRIMARY })),

                    ..base

                },

                button::Status::Pressed => button::Style {

                    background: Some(Background::Color(Color { a: 0.8, ..COL_PRIMARY })),

                    ..base

                },

                _ => base,

            }

        }

        

        fn secondary_button_style(_theme: &Theme, status: button::Status) -> button::Style {

            let base = button::Style {

                background: Some(Background::Color(Color::TRANSPARENT)),

                text_color: COL_PRIMARY,

                border: iced::Border {

                    radius: Radius::from(12.0),

                    width: 1.0,

                    color: COL_PRIMARY,

                },

                ..Default::default()

            };

            match status {

                button::Status::Hovered => button::Style {

                     background: Some(Background::Color(Color::from_rgba(0.36, 0.61, 0.61, 0.1))),

                    ..base

                },

                _ => base,

            }

        }

        

        

        pub fn run(debug: bool) -> iced::Result {

    iced::application("Yarm - Yet Another Resolution Manager", YarmApp::update, YarmApp::view)

        .theme(YarmApp::theme)

        .window(iced::window::Settings {

            size: iced::Size::new(720.0, 640.0), // Updated size

            ..Default::default()

        })

        .subscription(YarmApp::subscription)

        .run_with(move || YarmApp::new(debug))

}

struct YarmApp {
    monitors: Vec<Monitor>,
    config: AppConfig,
    staging_resolutions: std::collections::HashMap<String, Resolution>,
    new_profile_name: String,
    status_message: String,
    show_save_dialog: bool,
    debug: bool,
}

#[derive(Debug, Clone)]
enum Message {
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

impl Default for YarmApp {
    fn default() -> Self {
        Self {
            monitors: Vec::new(),
            config: AppConfig::default(),
            staging_resolutions: std::collections::HashMap::new(),
            new_profile_name: String::new(),
            status_message: "Loading...".to_string(),
            show_save_dialog: false,
            debug: false,
        }
    }
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

    fn subscription(&self) -> iced::Subscription<Message> {
        if self.debug {
            iced::event::listen_with(|event, _status, _window_id| {
                if let iced::Event::Window(iced::window::Event::Resized(size)) = event {
                    Some(Message::WindowResized(size))
                } else {
                    None
                }
            })
        } else {
            iced::Subscription::none()
        }
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

                self.config.profiles.retain(|p| p.name != self.new_profile_name);
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
            Message::WindowResized(size) => {
                if self.debug {
                    println!("Window resized to: {}x{}", size.width, size.height);
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // --- 1. Profiles Column (Floating Card) ---
        let profiles_list = self.config.profiles.iter().fold(column![].spacing(8), |col, profile| {
            col.push(
                button(
                    row![
                        text(&profile.name).size(16).color(COL_TEXT_DARK).width(Length::Fill),
                        text("×").size(18).color(COL_TEXT_MUTED)
                    ]
                    .align_y(Alignment::Center)
                )
                .on_press(Message::LoadProfile(profile.name.clone()))
                .width(Length::Fill)
                .padding(12)
                .style(|_theme, status| {
                    let mut base = button::Style {
                        background: Some(Background::Color(Color::TRANSPARENT)),
                        border: iced::Border { radius: Radius::from(8.0), ..Default::default() },
                        ..Default::default()
                    };
                    if status == button::Status::Hovered {
                        base.background = Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.03)));
                    }
                    base
                })
            )
        });

        let profiles_section = container(
            column![
                text("Profiles").size(22).font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }).color(COL_PRIMARY),
                scrollable(profiles_list).height(Length::Fill),
                vertical_space().height(10),
                button(text("+ Save Profile").align_x(iced::alignment::Horizontal::Center))
                    .on_press(Message::OpenSaveDialog)
                    .width(Length::Fill)
                    .padding(12)
                    .style(secondary_button_style)
            ]
            .spacing(15)
        )
        .width(Length::Fixed(200.0)) // Reduced width
        .height(Length::Fill)
        .padding(20)
        .style(floating_column_style);

        // --- 2. Main Monitor Area ---
        let monitors_list = self.monitors.iter().fold(column![].spacing(20), |col, monitor| {
            let current_staging = self.staging_resolutions.get(&monitor.id).unwrap_or(&monitor.current_resolution);
            let selected_option = monitor.available_resolutions.iter().find(|r| *r == current_staging).cloned();

            let control = container(
                pick_list(
                    &monitor.available_resolutions[..],
                    selected_option,
                    |res| Message::ResolutionChanged(monitor.id.clone(), res)
                )
                .width(Length::Fill)
                .padding(10)
            )
            .style(|_theme| container::Style {
                border: iced::Border {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    width: 1.0,
                    radius: Radius::from(12.0),
                },
                background: Some(Background::Color(Color::WHITE)),
                ..Default::default()
            });

            col.push(
                container(
                    column![
                        // Info
                        column![
                            row![
                                text(&monitor.name).size(18).font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }).color(COL_TEXT_DARK),
                                if monitor.is_primary {
                                    container(text("Primary").size(10).color(Color::WHITE))
                                        .padding([2, 6])
                                        .style(|_theme| container::Style {
                                            background: Some(Background::Color(COL_PRIMARY)),
                                            border: iced::Border { radius: Radius::from(10.0), ..Default::default() },
                                            ..Default::default()
                                        })
                                } else {
                                    container(text("")).width(0)
                                }
                            ].spacing(8).align_y(Alignment::Center),
                            
                            text(format!("ID: {}", monitor.id)).size(10).color(COL_TEXT_MUTED),
                            text(format!("Pos: ({}, {})", monitor.position.0, monitor.position.1)).size(10).color(COL_TEXT_MUTED),
                            
                            text(format!("{}Hz • {}bit", monitor.current_resolution.frequency, monitor.current_resolution.bits_per_pixel))
                                .size(12)
                                .color(COL_TEXT_MUTED),
                        ].spacing(4).width(Length::Fill),
                        
                        // Controls (Now below info)
                        column![
                            text("Target Resolution").size(12).color(COL_TEXT_MUTED),
                            control
                        ].spacing(5)
                    ]
                    .spacing(15)
                )
                .padding(20)
                .style(card_style)
            )
        });

        let status_indicator = {
            let color = if self.status_message.to_lowercase().contains("error") {
                Color::from_rgb(0.9, 0.4, 0.4)
            } else if self.status_message == "Ready" {
                Color::from_rgb(0.3, 0.8, 0.3)
            } else {
                COL_TEXT_MUTED
            };
            
            row![
                text("●").size(14).color(color),
                text(&self.status_message).size(14).color(COL_TEXT_MUTED)
            ].spacing(8).align_y(Alignment::Center)
        };

        let main_area = column![
            row![
                text("Monitors").size(28).font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }).color(COL_TEXT_DARK),
                horizontal_space(),
                status_indicator
            ].align_y(Alignment::Center),
            
            scrollable(monitors_list).height(Length::Fill),
            
            row![
                button("Refresh").on_press(Message::Refresh).style(secondary_button_style).padding(12),
                horizontal_space(),
                button("Apply Changes").on_press(Message::ApplyToSystem).style(primary_button_style).padding(12)
            ].align_y(Alignment::Center)
        ]
        .spacing(25)
        .padding(30)
        .width(Length::Fill);

        // --- 3. Assemble Layout ---
        let content = container(
            row![
                profiles_section,
                main_area
            ]
            .spacing(30)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(30) // Outer padding for floating feel
        .style(|_theme| container::Style {
            background: Some(Background::Color(COL_BACKGROUND)),
            ..Default::default()
        });

        // --- 4. Dialog Overlay ---
        if self.show_save_dialog {
            let monitor_summary = self.staging_resolutions.iter().fold(String::new(), |acc, (_, res)| {
                format!("{}• {}
", acc, res)
            });

            let dialog_card = container(
                column![
                    text("Profile").size(24).font(iced::Font { weight: iced::font::Weight::Bold, ..Default::default() }).color(COL_TEXT_DARK),
                    
                    text_input("Enter Profile Name", &self.new_profile_name)
                        .on_input(Message::NewProfileNameChanged)
                        .padding(10)
                        .size(16),
                        
                    container(
                        text(monitor_summary).size(12).color(COL_TEXT_MUTED)
                    )
                    .padding(10)
                    .style(|_theme| container::Style {
                        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.03))),
                        border: iced::Border { radius: Radius::from(8.0), ..Default::default() },
                        ..Default::default()
                    }),

                    row![
                        button(text("Cancel").align_x(iced::alignment::Horizontal::Center))
                            .on_press(Message::CloseSaveDialog)
                            .style(secondary_button_style)
                            .width(Length::Fill),
                        button(text("Confirm").align_x(iced::alignment::Horizontal::Center))
                            .on_press(Message::ConfirmSaveProfile)
                            .style(primary_button_style)
                            .width(Length::Fill),
                    ]
                    .spacing(10)
                ]
                .spacing(20)
            )
            .width(Length::Fixed(400.0))
            .padding(25)
            .style(card_style);

            stack![
                content,
                container(dialog_card)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .style(|_theme| container::Style {
                        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))), // Dimmed background
                        ..Default::default()
                    })
            ].into()
        } else {
            content.into()
        }
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