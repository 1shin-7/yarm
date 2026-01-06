use crate::ui::message::Message;
use crate::ui::state::YarmApp;
use iced::{event, Subscription};

pub fn subscription(app: &YarmApp) -> Subscription<Message> {
    if app.debug {
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
