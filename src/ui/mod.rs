pub mod loader;
pub mod message;
pub mod state;
pub mod subscription;
pub mod theme;
pub mod update;
pub mod view;
pub mod views;
pub mod widgets;

use crate::ui::loader::load_data;
use crate::ui::message::Message;
use crate::ui::state::YarmApp;
use iced::{Task, Theme};

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

    fn update(&mut self, message: Message) -> Task<Message> {
        update::update(self, message)
    }

    fn view(&self) -> iced::Element<'_, Message> {
        view::view(self)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        subscription::subscription(self)
    }

    fn theme(&self) -> Theme {
        Theme::Light
    }
}
