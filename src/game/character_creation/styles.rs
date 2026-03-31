use bevy::prelude::*;

pub(super) fn root_container_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::FlexStart,
        row_gap: Val::Px(18.0),
        padding: UiRect::axes(Val::Px(22.0), Val::Px(18.0)),
        ..default()
    }
}

pub(super) fn title_style() -> TextStyle {
    TextStyle {
        font_size: 52.0,
        color: Color::srgb_u8(236, 192, 112),
        ..default()
    }
}

pub(super) fn subtitle_style() -> TextStyle {
    TextStyle {
        font_size: 17.0,
        color: Color::srgb_u8(188, 160, 126),
        ..default()
    }
}

pub(super) fn character_creation_area_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        max_width: Val::Px(940.0),
        min_width: Val::Px(320.0),
        padding: UiRect::all(Val::Px(22.0)),
        border: UiRect::all(Val::Px(2.0)),
        flex_direction: FlexDirection::Column,
        ..default()
    }
}

pub(super) fn character_creation_area_bg_color() -> BackgroundColor {
    BackgroundColor(Color::srgba_u8(36, 20, 12, 228))
}

pub(super) fn character_creation_area_border_color() -> BorderColor {
    BorderColor(Color::srgb_u8(150, 106, 64))
}

pub(super) fn character_area_creation_area_row_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::SpaceBetween,
        column_gap: Val::Px(24.0),
        ..default()
    }
}

pub(super) fn character_area_creation_form_column_style() -> Style {
    Style {
        flex_grow: 1.0,
        min_width: Val::Px(280.0),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(8.0),
        ..default()
    }
}

pub(super) fn character_title_text_style() -> TextStyle {
    TextStyle {
        font_size: 18.0,
        color: Color::srgb_u8(222, 196, 156),
        ..default()
    }
}

pub(super) fn character_name_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Px(38.0),
        padding: UiRect::horizontal(Val::Px(10.0)),
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub(super) fn character_name_text_style() -> TextStyle {
    TextStyle {
        font_size: 16.0,
        color: Color::srgb_u8(196, 170, 136),
        ..default()
    }
}

pub(super) fn selector_row_title_text_style() -> TextStyle {
    TextStyle {
        font_size: 18.0,
        color: Color::srgb_u8(222, 196, 156),
        ..default()
    }
}

pub(super) fn selector_row_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        column_gap: Val::Px(8.0),
        ..default()
    }
}

pub(super) fn selector_row_prev_action_style() -> Style {
    Style {
        width: Val::Px(36.0),
        height: Val::Px(32.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub(super) fn selector_row_prev_action_bg_style() -> BackgroundColor {
    BackgroundColor(Color::srgba_u8(60, 38, 25, 230))
}

pub(super) fn selector_row_prev_action_text_style() -> TextStyle {
    TextStyle {
        font_size: 18.0,
        color: Color::srgb_u8(222, 196, 156),
        ..default()
    }
}

pub(super) fn selector_row_marker_text_style() -> TextStyle {
    TextStyle {
        font_size: 16.0,
        color: Color::srgb_u8(196, 170, 136),
        ..default()
    }
}

pub(super) fn selector_row_next_action_style() -> Style {
    Style {
        width: Val::Px(36.0),
        height: Val::Px(32.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub(super) fn selector_row_next_action_bg_style() -> BackgroundColor {
    BackgroundColor(Color::srgba_u8(60, 38, 25, 230))
}

pub(super) fn selector_row_next_action_text_style() -> TextStyle {
    TextStyle {
        font_size: 18.0,
        color: Color::srgb_u8(222, 196, 156),
        ..default()
    }
}

pub(super) fn creation_actions_container_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(10.0),
        row_gap: Val::Px(10.0),
        margin: UiRect::top(Val::Px(8.0)),
        ..default()
    }
}

pub(super) fn action_button_style() -> Style {
    Style {
        width: Val::Px(180.0),
        height: Val::Px(34.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub(super) fn action_button_bg_style() -> BackgroundColor {
    BackgroundColor(Color::srgba_u8(92, 56, 32, 240))
}

pub(super) fn action_button_text_style() -> TextStyle {
    TextStyle {
        font_size: 15.0,
        color: Color::srgb_u8(222, 196, 156),
        ..default()
    }
}
