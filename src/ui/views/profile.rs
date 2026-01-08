use crate::ui::model::Message;
use crate::ui::theme::{
    delete_icon_button_style, floating_column_style, secondary_button_style, COL_PRIMARY,
    COL_TEXT_DARK,
};
use crate::utils::config::Profile;
use iced::border::Radius;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

pub fn view<'a>(profiles: &'a [Profile]) -> Element<'a, Message> {
    let profiles_list = profiles.iter().fold(column![].spacing(8), |col, profile| {
        col.push(
            row![
                button(
                    text(&profile.name)
                        .size(16)
                        .color(COL_TEXT_DARK)
                        .width(Length::Fill)
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
                button(
                    text("×")
                        .size(18)
                        .align_x(iced::alignment::Horizontal::Center)
                        .align_y(iced::alignment::Vertical::Center)
                )
                .on_press(Message::RequestDeleteProfile(profile.name.clone()))
                .width(Length::Fixed(32.0))
                .height(Length::Fixed(32.0))
                .style(delete_icon_button_style)
            ]
            .align_y(Alignment::Center)
            .spacing(4),
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
