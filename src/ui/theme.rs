use iced::border::Radius;
use iced::widget::{button, container};
use iced::{Background, Color, Shadow, Theme, Vector};

// --- Theme Colors (Sea Salt Blue Palette) ---
pub const COL_BACKGROUND: Color = Color::from_rgb(0.96, 0.97, 0.98); // Very light gray-blue
pub const COL_SURFACE: Color = Color::from_rgb(0.98, 0.98, 0.98); // Almost white/light gray
pub const COL_PRIMARY: Color = Color::from_rgb(0.36, 0.61, 0.61); // Sea Salt Blue / Muted Teal
pub const COL_PRIMARY_TEXT: Color = Color::from_rgb(1.0, 1.0, 1.0);
pub const COL_TEXT_DARK: Color = Color::from_rgb(0.2, 0.25, 0.3);
pub const COL_TEXT_MUTED: Color = Color::from_rgb(0.5, 0.55, 0.6);

// --- Style Helpers ---
pub fn card_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))), // Light gray
        border: iced::Border {
            width: 0.0,
            color: Color::TRANSPARENT,
            radius: Radius::from(12.0),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.03),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        ..Default::default()
    }
}

pub fn floating_column_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(COL_SURFACE)), // Simulate glass/floating
        border: iced::Border {
            width: 1.0,
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.6), // Subtle highlight border
            radius: Radius::from(12.0),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            offset: Vector::new(0.0, 8.0), // Higher float
            blur_radius: 16.0,
        },
        ..Default::default()
    }
}

pub fn primary_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(COL_PRIMARY)),
        text_color: COL_PRIMARY_TEXT,
        border: iced::Border {
            radius: Radius::from(12.0),
            ..Default::default()
        },
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color {
                a: 0.9,
                ..COL_PRIMARY
            })),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color {
                a: 0.8,
                ..COL_PRIMARY
            })),
            ..base
        },
        _ => base,
    }
}

pub fn secondary_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: COL_PRIMARY,
        border: iced::Border {
            radius: Radius::from(12.0),
            width: 1.0,
            color: COL_PRIMARY,
        },
        ..Default::default()
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgba(0.36, 0.61, 0.61, 0.1))),
            ..base
        },
        _ => base,
    }
}
