use bevy::prelude::*;

pub fn title_style() -> TextStyle {
    TextStyle {
        font_size: 56.0,
        color: Color::srgb_u8(236, 192, 112),
        ..default()
    }
}

pub fn subtitle_style() -> TextStyle {
    TextStyle {
        font_size: 18.0,
        color: Color::srgb_u8(188, 160, 126),
        ..default()
    }
}
