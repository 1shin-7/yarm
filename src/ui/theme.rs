use iced::border::Radius;
use iced::widget::{button, container, pick_list};
use iced::{Background, Border, Color, Shadow, Theme, Vector};

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
        border: Border {
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
        border: Border {
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
        border: Border {
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
        border: Border {
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

pub fn pick_list_style(_theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let active = pick_list::Style {
        text_color: COL_TEXT_DARK,
        placeholder_color: COL_TEXT_MUTED,
        handle_color: COL_TEXT_MUTED,
        background: Background::Color(Color::WHITE),
        border: Border {
            radius: Radius::from(12.0),
            width: 1.0,
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
        },
    };

    match status {
        pick_list::Status::Active => active,
        pick_list::Status::Hovered => pick_list::Style {
            border: Border {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                ..active.border
            },
            ..active
        },
        pick_list::Status::Opened => pick_list::Style {
            border: Border {
                color: COL_PRIMARY,
                ..active.border
            },
            ..active
        },
    }
}

// Menu style is usually defined via the `menu` method on PickList logic or passed implicitly if supported in theme?
// In Iced 0.13, PickList takes a style function that returns `pick_list::Style`.
// `pick_list::Style` has NO `menu` field in older versions, but let's check recent.
// If it doesn't, we rely on the Theme's default menu style or we need a way to style menu.
// Actually, `pick_list` widget function in 0.13 does not take a separate menu style.
// Wait, `pick_list::Style` might not expose menu customization directly in the return of that function depending on version.
// Let's assume standard behavior: we return the widget style.
// If we can't style the menu easily without a custom theme implementation, we'll stick to the widget style.
// However, looking at source code for 0.12+, `pick_list` style fn returns `Style`.
