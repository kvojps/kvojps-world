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
