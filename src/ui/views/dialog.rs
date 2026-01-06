use crate::display::Resolution;
use crate::ui::message::Message;
use crate::ui::theme::{
    card_style, primary_button_style, secondary_button_style, COL_TEXT_DARK, COL_TEXT_MUTED,
};
use iced::border::Radius;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Background, Color, Element, Length};
use std::collections::HashMap;

pub fn view<'a>(
    show: bool,
    new_profile_name: &'a str,
    staging_resolutions: &'a HashMap<String, Resolution>,
    content: Element<'a, Message>,
) -> Element<'a, Message> {
    if show {
        let monitor_summary = staging_resolutions
            .iter()
            .fold(String::new(), |acc, (_, res)| {
                format!(
                    "{}• {}
",
                    acc, res
                )
            });

        let dialog_card = container(
            column![
                text("Profile")
                    .size(24)
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(COL_TEXT_DARK),
                text_input("Enter Profile Name", new_profile_name)
                    .on_input(Message::NewProfileNameChanged)
                    .padding(10)
                    .size(16),
                container(text(monitor_summary).size(12).color(COL_TEXT_MUTED))
                    .padding(10)
                    .style(|_theme| container::Style {
                        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.03))),
                        border: iced::Border {
                            radius: Radius::from(8.0),
                            ..Default::default()
                        },
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
            .spacing(20),
        )
        .width(Length::Fixed(400.0))
        .padding(25)
        .style(card_style)
        .width(Length::Fill);

        iced::widget::stack![
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
        ]
        .into()
    } else {
        content
    }
}
