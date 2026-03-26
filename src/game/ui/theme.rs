use bevy::prelude::Color;

pub const MENU_BACKGROUND_SPEED: f32 = 0.5;
pub const CREATION_BACKGROUND_SPEED: f32 = 0.55;

pub const FONT_SIZE_DISPLAY_TITLE: f32 = 56.0;
pub const FONT_SIZE_SCREEN_TITLE: f32 = 52.0;
pub const FONT_SIZE_MENU_OPTION: f32 = 28.0;
pub const FONT_SIZE_LABEL: f32 = 18.0;
pub const FONT_SIZE_BODY: f32 = 17.0;
pub const FONT_SIZE_VALUE: f32 = 16.0;
pub const FONT_SIZE_HINT: f32 = 15.0;
pub const FONT_SIZE_FOOTER: f32 = 14.0;
pub const FONT_SIZE_STATUS: f32 = 13.0;

pub fn ui_title_text() -> Color {
    Color::srgb_u8(236, 192, 112)
}

pub fn ui_subtitle_text() -> Color {
    Color::srgb_u8(188, 160, 126)
}

pub fn ui_label_text() -> Color {
    Color::srgb_u8(222, 196, 156)
}

pub fn ui_value_text() -> Color {
    Color::srgb_u8(196, 170, 136)
}

pub fn ui_hint_text() -> Color {
    Color::srgb_u8(236, 168, 104)
}

pub fn ui_muted_text() -> Color {
    Color::srgb_u8(152, 128, 102)
}

pub fn ui_selected_text() -> Color {
    Color::srgb_u8(252, 204, 118)
}

pub fn ui_error_text() -> Color {
    Color::srgb_u8(240, 130, 110)
}

pub fn ui_panel_background() -> Color {
    Color::srgba_u8(36, 20, 12, 228)
}

pub fn ui_panel_border() -> Color {
    Color::srgb_u8(150, 106, 64)
}

pub fn ui_button_background() -> Color {
    Color::srgba_u8(18, 10, 6, 200)
}

pub fn ui_button_selected_background() -> Color {
    Color::srgba_u8(74, 44, 26, 230)
}

pub fn ui_name_input_background() -> Color {
    Color::srgba_u8(18, 12, 8, 220)
}

pub fn ui_name_input_active_background() -> Color {
    Color::srgba_u8(94, 58, 36, 245)
}

pub fn ui_selector_button_background() -> Color {
    Color::srgba_u8(60, 38, 25, 230)
}

pub fn ui_action_button_background() -> Color {
    Color::srgba_u8(92, 56, 32, 240)
}

pub fn ui_portrait_frame_background() -> Color {
    Color::srgba_u8(18, 10, 6, 170)
}

pub fn ember_background(pulse: f32) -> Color {
    Color::srgb(
        0.06 + 0.03 * pulse,
        0.03 + 0.015 * pulse,
        0.02 + 0.01 * pulse,
    )
}
