use crate::display::{Monitor, Resolution};
use crate::ui::message::Message;
use crate::ui::theme::{
    card_style, primary_button_style, secondary_button_style, COL_PRIMARY, COL_TEXT_DARK,
    COL_TEXT_MUTED,
};
use iced::border::Radius;
use iced::widget::{button, column, container, pick_list, row, scrollable, text};
use iced::{Alignment, Background, Color, Element, Length};
use std::collections::HashMap;

pub fn view<'a>(
    monitors: &'a [Monitor],
    staging_resolutions: &'a HashMap<String, Resolution>,
    status_message: &'a str,
) -> Element<'a, Message> {
    let monitors_list = monitors.iter().fold(column![].spacing(20), |col, monitor| {
        let current_staging = staging_resolutions
            .get(&monitor.id)
            .unwrap_or(&monitor.current_resolution);
        let selected_option = monitor
            .available_resolutions
            .iter()
            .find(|r| *r == current_staging)
            .cloned();

        let control = container(
            pick_list(&monitor.available_resolutions[..], selected_option, |res| {
                Message::ResolutionChanged(monitor.id.clone(), res)
            })
            .width(Length::Fill)
            .padding(10),
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
                            text(&monitor.name)
                                .size(18)
                                .font(iced::Font {
                                    weight: iced::font::Weight::Bold,
                                    ..Default::default()
                                })
                                .color(COL_TEXT_DARK),
                            if monitor.is_primary {
                                container(text("Primary").size(10).color(Color::WHITE))
                                    .padding([2, 6])
                                    .style(|_theme| container::Style {
                                        background: Some(Background::Color(COL_PRIMARY)),
                                        border: iced::Border {
                                            radius: Radius::from(10.0),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                            } else {
                                container(text("")).width(0)
                            }
                        ]
                        .spacing(8)
                        .align_y(Alignment::Center),
                        text(format!("ID: {}", monitor.id))
                            .size(10)
                            .color(COL_TEXT_MUTED),
                        text(format!(
                            "Pos: ({}, {})",
                            monitor.position.0, monitor.position.1
                        ))
                        .size(10)
                        .color(COL_TEXT_MUTED),
                        text(format!(
                            "{}Hz • {}bit",
                            monitor.current_resolution.frequency,
                            monitor.current_resolution.bits_per_pixel
                        ))
                        .size(12)
                        .color(COL_TEXT_MUTED),
                    ]
                    .spacing(4)
                    .width(Length::Fill),
                    // Controls
                    column![
                        text("Target Resolution").size(12).color(COL_TEXT_MUTED),
                        control
                    ]
                    .spacing(5)
                ]
                .spacing(15),
            )
            .padding(20)
            .style(card_style),
        )
    });

    let status_indicator = {
        let color = if status_message.to_lowercase().contains("error") {
            Color::from_rgb(0.9, 0.4, 0.4)
        } else if status_message == "Ready" {
            Color::from_rgb(0.3, 0.8, 0.3)
        } else {
            COL_TEXT_MUTED
        };

        row![
            text("●").size(14).color(color),
            text(status_message).size(14).color(COL_TEXT_MUTED)
        ]
        .spacing(8)
        .align_y(Alignment::Center)
    };

    column![
        row![
            text("Monitors")
                .size(28)
                .font(iced::Font {
                    weight: iced::font::Weight::Bold,
                    ..Default::default()
                })
                .color(COL_TEXT_DARK),
            iced::widget::horizontal_space(),
            status_indicator
        ]
        .align_y(Alignment::Center),
        scrollable(monitors_list).height(Length::Fill),
        row![
            button("Refresh")
                .on_press(Message::Refresh)
                .style(secondary_button_style)
                .padding(12),
            iced::widget::horizontal_space(),
            button("Apply Changes")
                .on_press(Message::ApplyToSystem)
                .style(primary_button_style)
                .padding(12)
        ]
        .align_y(Alignment::Center)
    ]
    .spacing(25)
    .padding(30)
    .width(Length::Fill)
    .into()
}
