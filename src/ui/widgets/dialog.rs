use crate::ui::model::Message;
use crate::ui::theme::{card_style, COL_TEXT_DARK};
use iced::widget::{column, container, row, text};
use iced::{Background, Color, Element, Length};

pub fn view<'a>(
    show: bool,
    title: &'a str,
    modal_content: Element<'a, Message>,
    buttons: Vec<Element<'a, Message>>,
    base_content: Element<'a, Message>,
) -> Element<'a, Message> {
    if show {
        let mut actions = row![].spacing(10).width(Length::Fill);
        for btn in buttons {
            actions = actions.push(btn);
        }

        let dialog_card = container(
            column![
                text(title)
                    .size(24)
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..Default::default()
                    })
                    .color(COL_TEXT_DARK),
                modal_content,
                actions
            ]
            .spacing(20),
        )
        .width(Length::Fixed(400.0))
        .padding(25)
        .style(card_style);

        iced::widget::stack![
            base_content,
            container(dialog_card)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
                    ..Default::default()
                })
        ]
        .into()
    } else {
        base_content
    }
}
