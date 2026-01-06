use crate::ui::message::Message;
use crate::ui::state::YarmApp;
use crate::ui::theme::COL_BACKGROUND;
use crate::ui::views;
use iced::widget::{container, row};
use iced::{Background, Element, Length};

pub fn view(app: &YarmApp) -> Element<'_, Message> {

    let profiles_section = views::profile::view(&app.config.profiles);

    let main_area = views::monitor::view(

        &app.monitors,

        &app.staging_resolutions,

        &app.staging_orientations,

        &app.status_message

    );



    let content = container(row![profiles_section, main_area].spacing(30))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(30) // Outer padding for floating feel
        .style(|_theme| container::Style {
            background: Some(Background::Color(COL_BACKGROUND)),
            ..Default::default()
        });

    views::dialog::view(
        app.show_save_dialog,
        &app.new_profile_name,
        &app.staging_resolutions,
        content.into(),
    )
}
