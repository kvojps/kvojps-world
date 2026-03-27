use super::components::MainMenuUiRoot;
use crate::themes::*;
use bevy::prelude::*;

pub(super) fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    row_gap: Val::Px(16.0),
                    padding: UiRect::axes(Val::Px(18.0), Val::Px(20.0)),
                    ..default()
                },
                ..default()
            },
            MainMenuUiRoot,
        ))
        .with_children(|root| {
            _spawn_header(root);
        });
}

fn _spawn_header(root: &mut ChildBuilder) {
    let title_style = title_style();
    let subtitle_style = subtitle_style();

    root.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        },
        ..default()
    })
    .with_children(|header| {
        header.spawn(TextBundle::from_section(
            "Kvojps World",
            title_style.clone(),
        ));
        header.spawn(TextBundle::from_section(
            "Uma aventura RPG de sobrevivencia e exploracao",
            subtitle_style.clone(),
        ));
    });
}

pub(super) fn cleanup_main_menu(
    mut commands: Commands,
    roots: Query<Entity, With<MainMenuUiRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn_recursive();
    }
}
