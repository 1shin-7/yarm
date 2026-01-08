use crate::display::{Monitor, Orientation, Resolution};
use crate::ui::model::Message;
use crate::ui::theme::{
    card_style, pick_list_style, settings_icon_button_style, COL_PRIMARY, COL_TEXT_DARK, COL_TEXT_MUTED,
};
use crate::ui::widgets::orientation_switcher::OrientationSwitcher;
use iced::border::Radius;
use iced::widget::{button, column, container, pick_list, row, scrollable, svg, text};
use iced::{Alignment, Background, Color, Element, Length};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Dimension {
    width: u32,
    height: u32,
}

impl std::fmt::Display for Dimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

pub fn view<'a>(
    monitors: &'a [Monitor],
    staging_resolutions: &'a HashMap<String, Resolution>,
    staging_orientations: &'a HashMap<String, Orientation>,
    status_message: &'a str,
) -> Element<'a, Message> {
    let monitors_list = monitors.iter().fold(column![].spacing(20), |col, monitor| {
        let current_res_staging = staging_resolutions
            .get(&monitor.id)
            .unwrap_or(&monitor.current_resolution);

        let current_orient_staging = staging_orientations
            .get(&monitor.id)
            .unwrap_or(&monitor.current_orientation);

        // 1. Prepare Dimensions
        let mut available_dims: Vec<Dimension> = monitor
            .available_resolutions
            .iter()
            .map(|r| Dimension {
                width: r.width,
                height: r.height,
            })
            .collect();
        available_dims.sort_by(|a, b| (b.width * b.height).cmp(&(a.width * a.height))); // Sort by area desc
        available_dims.dedup();

        let current_dim = Dimension {
            width: current_res_staging.width,
            height: current_res_staging.height,
        };
        
        let selected_dim = available_dims.iter().find(|d| **d == current_dim).cloned();

        // 2. Prepare Frequencies for current dimension
        let mut available_freqs: Vec<u32> = monitor
            .available_resolutions
            .iter()
            .filter(|r| r.width == current_dim.width && r.height == current_dim.height)
            .map(|r| r.frequency)
            .collect();
        available_freqs.sort_by(|a, b| b.cmp(a)); // Descending
        available_freqs.dedup();

        let current_freq = current_res_staging.frequency;
        let selected_freq = available_freqs.iter().find(|f| **f == current_freq).cloned();

        // Controls
        let dim_pick_list = pick_list(
            available_dims,
            selected_dim,
            {
                let id = monitor.id.clone();
                let available_resolutions = monitor.available_resolutions.clone();
                move |new_dim| {
                    // Fallback logic: Pick highest frequency for new dim
                    // Also need to handle bits_per_pixel. We'll pick the "best" resolution object matching dim and max freq.
                    let best_match = available_resolutions.iter()
                        .filter(|r| r.width == new_dim.width && r.height == new_dim.height)
                        .max_by_key(|r| (r.frequency, r.bits_per_pixel)) // Prefer high freq, then high depth
                        .cloned();
                    
                    if let Some(res) = best_match {
                        Message::ResolutionChanged(id.clone(), res)
                    } else {
                        // Should not happen if available_dims came from available_resolutions
                        Message::RefreshTick
                    }
                }
            }
        )
        .width(Length::Fill)
        .padding(12)
        .style(pick_list_style);

        let freq_pick_list = pick_list(
            available_freqs,
            selected_freq,
            {
                let id = monitor.id.clone();
                let available_resolutions = monitor.available_resolutions.clone();
                move |new_freq| {
                    // Find resolution matching current dim and new freq. Prefer highest bit depth.
                    let best_match = available_resolutions.iter()
                        .filter(|r| r.width == current_dim.width && r.height == current_dim.height && r.frequency == new_freq)
                        .max_by_key(|r| r.bits_per_pixel)
                        .cloned();

                    if let Some(res) = best_match {
                        Message::ResolutionChanged(id.clone(), res)
                    } else {
                        Message::RefreshTick
                    }
                }
            }
        )
        .width(Length::Fixed(100.0)) // Fixed width for frequency
        .padding(12)
        .style(pick_list_style);

        let orient_control = OrientationSwitcher::new(*current_orient_staging, {
            let id = monitor.id.clone();
            move |orient| Message::OrientationChanged(id.clone(), orient)
        })
        .view();

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
                                            radius: Radius::from(12.0),
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
                        // Line 1: Specs
                        text(format!(
                            "{}Hz • {}bit",
                            monitor.current_resolution.frequency,
                            monitor.current_resolution.bits_per_pixel
                        ))
                        .size(12)
                        .color(COL_TEXT_MUTED),
                        // Line 2: ID & Pos
                        text(format!(
                            "ID: {} • Pos: ({}, {})",
                            monitor.id, monitor.position.0, monitor.position.1
                        ))
                        .size(12)
                        .color(COL_TEXT_MUTED),
                    ]
                    .spacing(4)
                    .width(Length::Fill),
                    // Controls
                    column![
                        text("Resolution").size(12).color(COL_TEXT_MUTED),
                        row![
                            dim_pick_list,
                            freq_pick_list
                        ].spacing(10),
                        text("Orientation").size(12).color(COL_TEXT_MUTED),
                        orient_control
                    ]
                    .spacing(8)
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
            button(
                svg(svg::Handle::from_memory(include_bytes!("../../../assets/icons/cog.svg").as_slice()))
                    .width(Length::Fixed(20.0))
                    .height(Length::Fixed(20.0))
            )
            .on_press(Message::OpenSettings)
            .style(settings_icon_button_style)
            .padding(6)
            .width(Length::Fixed(32.0))
            .height(Length::Fixed(32.0)),
            iced::widget::horizontal_space(),
            status_indicator
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        scrollable(monitors_list).height(Length::Fill),
    ]
    .spacing(25)
    .padding(iced::Padding {
        top: 0.0,
        right: 30.0,
        bottom: 30.0,
        left: 30.0,
    })
    .width(Length::Fill)
    .into()
}
