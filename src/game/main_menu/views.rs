use super::components::MainMenuUiRoot;
use super::styles::{header_container_style, root_container_style};
use crate::themes::*;
use bevy::prelude::*;

pub(super) fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: root_container_style(),
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
        style: header_container_style(),
        ..default()
    })
    .with_children(|header| {
        header.spawn(TextBundle::from_section(
            "Kvojps World",
            title_style.clone(),
        ));
        header.spawn(TextBundle::from_section(
            "Uma aventura RPG de sobrevivência e exploracão",
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
