use crate::display::Orientation;
use crate::ui::theme::{COL_PRIMARY, COL_PRIMARY_TEXT, COL_TEXT_MUTED};
use iced::border::Radius;
use iced::widget::{button, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Theme};

pub struct OrientationSwitcher<Message> {
    current: Orientation,
    on_change: Box<dyn Fn(Orientation) -> Message>,
}

impl<Message: Clone + 'static> OrientationSwitcher<Message> {
    pub fn new(current: Orientation, on_change: impl Fn(Orientation) -> Message + 'static) -> Self {
        Self {
            current,
            on_change: Box::new(on_change),
        }
    }

    pub fn view(self) -> Element<'static, Message> {
        let options = [
            (Orientation::Landscape, "0°"),
            (Orientation::Portrait, "90°"),
            (Orientation::LandscapeFlipped, "180°"),
            (Orientation::PortraitFlipped, "270°"),
        ];

        let content = options.iter().fold(row![], |row, (orientation, label)| {
            let is_selected = *orientation == self.current;

            let btn_style = move |_theme: &Theme, status: button::Status| {
                let mut style = button::Style {
                    border: Border {
                        radius: Radius::from(6.0),
                        width: 0.0,
                        color: Color::TRANSPARENT,
                    },
                    ..Default::default()
                };
                if is_selected {
                    style.background = Some(Background::Color(COL_PRIMARY));
                    style.text_color = COL_PRIMARY_TEXT;
                } else {
                    style.background = Some(Background::Color(Color::TRANSPARENT));
                    style.text_color = COL_TEXT_MUTED;
                    if status == button::Status::Hovered {
                        style.background =
                            Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.05)));
                    }
                }
                style
            };

            let mut btn = button(
                container(text(*label).size(12))
                    .width(Length::Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding(8)
            .style(btn_style);

            if !is_selected {
                btn = btn.on_press((self.on_change)(*orientation));
            }

            // Add separator line if not last and not selected (optional, but grouped look is cleaner without internal borders if highlighting is used)
            // For shadcn grouped style, usually just background highlight.

            row.push(btn)
        });

        container(content)
            .width(Length::Fill)
            .padding(4) // Add padding so highlighted button isn't flush with edges
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::WHITE)),
                border: Border {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    width: 1.0,
                    radius: Radius::from(8.0),
                },
                ..Default::default()
            })
            .into()
    }
}
