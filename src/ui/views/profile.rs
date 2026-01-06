use crate::ui::message::Message;
use crate::ui::theme::{
    floating_column_style, secondary_button_style, COL_PRIMARY, COL_TEXT_DARK, COL_TEXT_MUTED,
};
use crate::utils::config::Profile;
use iced::border::Radius;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

pub fn view<'a>(profiles: &'a [Profile]) -> Element<'a, Message> {
    let profiles_list = profiles.iter().fold(column![].spacing(8), |col, profile| {
        col.push(
            button(
                row![
                    text(&profile.name)
                        .size(16)
                        .color(COL_TEXT_DARK)
                        .width(Length::Fill),
                    text("×").size(18).color(COL_TEXT_MUTED)
                ]
                .align_y(Alignment::Center),
            )
            .on_press(Message::LoadProfile(profile.name.clone()))
            .width(Length::Fill)
            .padding(12)
            .style(|_theme, status| {
                let mut base = button::Style {
                    background: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
                    border: iced::Border {
                        radius: Radius::from(8.0),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                if status == button::Status::Hovered {
                    base.background = Some(iced::Background::Color(iced::Color::from_rgba(
                        0.0, 0.0, 0.0, 0.03,
                    )));
                }
                base
            }),
        )
    });

    container(
        column![
            text("Profiles")
                .size(22)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(COL_PRIMARY),
            scrollable(profiles_list).height(Length::Fill),
            iced::widget::vertical_space().height(10),
            button(text("+ Save Profile").align_x(iced::alignment::Horizontal::Center))
                .on_press(Message::OpenSaveDialog)
                .width(Length::Fill)
                .padding(12)
                .style(secondary_button_style)
        ]
        .spacing(15),
    )
    .width(Length::Fixed(200.0))
    .height(Length::Fill)
    .padding(20)
    .style(floating_column_style)
    .into()
}
