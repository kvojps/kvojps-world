use bevy::prelude::Color;

pub const MENU_BACKGROUND_SPEED: f32 = 0.5;
pub const CREATION_BACKGROUND_SPEED: f32 = 0.55;

pub fn ember_background(pulse: f32) -> Color {
    Color::srgb(
        0.06 + 0.03 * pulse,
        0.03 + 0.015 * pulse,
        0.02 + 0.01 * pulse,
    )
}
