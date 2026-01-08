use crate::ui::theme::{backdrop_style, card_style, COL_TEXT_DARK};
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};

pub fn view<'a, Message>(
    show: bool,
    title: &'a str,
    modal_content: Element<'a, Message>,
    buttons: Vec<Element<'a, Message>>,
    on_backdrop: Option<Message>,
    base_content: Element<'a, Message>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    if show {
        let mut buttons_row = row![].spacing(8).width(Length::Fill).height(Length::Fixed(40.0));
        for btn in buttons {
            buttons_row = buttons_row.push(btn);
        }

        let dialog_card = container(
            column![
                // Title Area
                container(
                    text(title)
                        .size(20)
                        .font(iced::Font {
                            weight: iced::font::Weight::Bold,
                            ..Default::default()
                        })
                        .color(COL_TEXT_DARK)
                )
                .width(Length::Fill)
                .padding(iced::Padding {
                    top: 12.0,
                    right: 0.0,
                    bottom: 12.0,
                    left: 0.0,
                })
                .align_x(Alignment::Center),

                // Content Area
                container(modal_content)
                    .width(Length::Fill)
                    .padding(iced::Padding {
                        top: 0.0,
                        right: 12.0,
                        bottom: 12.0,
                        left: 12.0,
                    }),

                // Action Area (Buttons)
                buttons_row
            ]
        )
        .width(Length::Fixed(400.0))
        .padding(8) // Tight 8px padding to border
        .style(card_style);

        // Backdrop: A full-screen button that consumes all events.
        let mut backdrop = button(text(" ")) 
            .width(Length::Fill)
            .height(Length::Fill)
            .style(backdrop_style);

        if let Some(msg) = on_backdrop {
            backdrop = backdrop.on_press(msg);
        }

        iced::widget::stack![
            base_content,
            backdrop,
            container(dialog_card)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
        ]
        .into()
    } else {
        base_content
    }
}
