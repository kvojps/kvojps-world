use bevy::prelude::Color;
use bevy_egui::egui::Color32;

pub const MENU_BACKGROUND_SPEED: f32 = 0.5;
pub const CREATION_BACKGROUND_SPEED: f32 = 0.55;

pub fn ember_background(pulse: f32) -> Color {
    Color::srgb(
        0.06 + 0.03 * pulse,
        0.03 + 0.015 * pulse,
        0.02 + 0.01 * pulse,
    )
}

pub fn surface_fill() -> Color32 {
    Color32::from_rgba_unmultiplied(18, 10, 6, 0)
}

pub fn panel_fill() -> Color32 {
    Color32::from_rgba_unmultiplied(36, 20, 12, 228)
}

pub fn panel_stroke() -> Color32 {
    Color32::from_rgb(150, 106, 64)
}

pub fn title_gold() -> Color32 {
    Color32::from_rgb(236, 192, 112)
}

pub fn subtitle_parchment() -> Color32 {
    Color32::from_rgb(188, 160, 126)
}

pub fn text_primary() -> Color32 {
    Color32::from_rgb(222, 196, 156)
}

pub fn text_body() -> Color32 {
    Color32::from_rgb(196, 170, 136)
}

pub fn text_muted() -> Color32 {
    Color32::from_rgb(152, 128, 102)
}

pub fn accent_ember() -> Color32 {
    Color32::from_rgb(236, 168, 104)
}

pub fn selection_gold() -> Color32 {
    Color32::from_rgb(252, 204, 118)
}

pub fn error_red() -> Color32 {
    Color32::from_rgb(240, 130, 110)
}
