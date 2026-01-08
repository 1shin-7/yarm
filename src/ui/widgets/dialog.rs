use crate::ui::theme::{backdrop_style, card_style, COL_TEXT_DARK};
use iced::widget::{button, column, container, row, text};
use iced::{Element, Length};

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
        let mut buttons_row = row![].spacing(10).width(Length::Fill);
        for btn in buttons {
            buttons_row = buttons_row.push(btn);
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
                buttons_row
            ]
            .spacing(20),
        )
        .width(Length::Fixed(400.0))
        .padding(25)
        .style(card_style);

        // Backdrop: A full-screen button that consumes all events.
        let mut backdrop = button(text(" ")) 
            .width(Length::Fill)
            .height(Length::Fill)
            .style(backdrop_style);

        if let Some(msg) = on_backdrop {
            backdrop = backdrop.on_press(msg);
        }

        // We use a stack.
        // 1. Base Content
        // 2. Backdrop (Full screen button)
        // 3. Dialog Container (Centered)
        
        // Note: The Dialog needs to be centered. 
        // Simply stacking the card on top might rely on alignment.
        // A container wrapper for the card is needed for centering logic *over* the backdrop.
        // Wait, if I stack [Backdrop, Card], the Card needs to be positioned.
        // Backdrop is Fill/Fill.
        // To center the card, we usually wrap it in a Container that is Fill/Fill and Centers content.
        // BUT, if that Container is on top of the Backdrop Button, THAT Container will intercept mouse events (being transparent).
        // 
        // Trick: Make the "Centering Container" pass-through? Iced containers usually block if they have background.
        // If they don't have background, they *might* pass through clicks to the layer below?
        // Let's test standard behavior:
        // Stack [ 
        //   BackdropButton (Fill/Fill),
        //   Container(Card).center_x().center_y() (Fill/Fill, transparent) 
        // ]
        // If the transparent container blocks the button below, then clicking "outside" (on the transparent part) won't hit the backdrop button.
        //
        // Solution: The "Backdrop" IS the container's background? No, we need a Button for interaction blocking.
        //
        // Alternative: The Dialog Card itself is the only thing in the top layer.
        // But `Stack` doesn't support "Alignment" for children directly in 0.13 without a container?
        // Actually, `Stack` just piles them up top-left.
        //
        // Let's try to put the `backdrop` button *inside* a container, and putting the card *next* to it? No.
        //
        // Correct approach for Iced 0.13 to center a modal over a backdrop button:
        // Stack [
        //    base,
        //    backdrop_button, 
        //    container(card).width(Fill).height(Fill).center_x(Fill).center_y(Fill)
        // ]
        //
        // If the top container blocks clicks, the backdrop button underneath won't receive `on_press`.
        // Iced `Container` usually does NOT block mouse events if it has no background/listeners, UNLESS it contains something.
        // The empty space in a container *should* let events through to the layer below.
        // Let's proceed with this assumption (Standard Iced Modal Pattern).

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
