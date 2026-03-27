use bevy::prelude::*;

pub(super) fn root_container_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::FlexStart,
        row_gap: Val::Px(16.0),
        padding: UiRect::axes(Val::Px(18.0), Val::Px(20.0)),
        ..default()
    }
}

pub(super) fn header_container_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        margin: UiRect::top(Val::Px(20.0)),
        ..default()
    }
}

pub(super) fn menu_panel_container_style() -> Style {
    Style {
        width: Val::Px(460.0),
        max_width: Val::Percent(100.0),
        padding: UiRect::all(Val::Px(18.0)),
        border: UiRect::all(Val::Px(2.0)),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(8.0),
        margin: UiRect::top(Val::Px(24.0)),
        ..default()
    }
}

pub(super) fn menu_item_style() -> Style {
    Style {
        width: Val::Percent(100.0),
        height: Val::Px(42.0),
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Center,
        padding: UiRect::left(Val::Px(10.0)),
        ..default()
    }
}

pub(super) fn menu_item_colors() -> (BackgroundColor, BorderColor) {
    (
        BackgroundColor(Color::srgb_u8(150, 106, 64)),
        BorderColor(Color::NONE),
    )
}

pub(super) fn menu_item_label_style() -> TextStyle {
    TextStyle {
        font_size: 18.0,
        color: Color::srgb_u8(222, 196, 156),
        ..default()
    }
}
