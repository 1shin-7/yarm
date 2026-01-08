use crate::ui::model::Message;
use crate::ui::theme::{
    compact_neutral_button_style, delete_icon_button_style, floating_column_style, COL_PRIMARY,
    COL_TEXT_DARK,
};
use crate::utils::config::Profile;
use iced::border::Radius;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

pub fn view<'a>(profiles: &'a [Profile]) -> Element<'a, Message> {
    let profiles_list = profiles.iter().fold(column![].spacing(8), |col, profile| {
        // ... (existing profiles_list logic)
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
                            radius: Radius::from(12.0),
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
            // Header and List with standard padding
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
                ]
                .spacing(15)
            )
            .height(Length::Fill)
            .padding(iced::Padding {
                top: 20.0,
                right: 20.0,
                bottom: 10.0,
                left: 20.0,
            }),
            // Footer with Save button and tighter padding
            container(
                button(
                    text("Save")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .align_x(Alignment::Center)
                        .align_y(Alignment::Center)
                        .font(iced::Font {
                            weight: iced::font::Weight::Semibold,
                            ..Default::default()
                        })
                )
                .on_press(Message::OpenSaveDialog)
                .width(Length::Fill)
                .height(Length::Fixed(40.0))
                .padding(0)
                .style(compact_neutral_button_style)
            )
            .padding(8)
        ]
    )
    .width(Length::Fixed(200.0))
    .height(Length::Fill)
    .style(floating_column_style)
    .into()
}
