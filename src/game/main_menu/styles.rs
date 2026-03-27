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
