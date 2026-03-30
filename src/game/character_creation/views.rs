use super::components::*;
use super::styles::*;
use bevy::prelude::*;

pub(super) fn setup_character_creation(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: root_container_style(),
                ..default()
            },
            CharacterCreationUiRoot,
        ))
        .with_children(|root| {
            _spawn_creation_header(root);
            _spawn_character_creation_area(root);
        });
}

fn _spawn_creation_header(root: &mut ChildBuilder) {
    root.spawn(TextBundle::from_section("Sala da Guilda", title_style()));
    root.spawn(TextBundle::from_section(
        "Registre seu aventureiro antes de cruzar os portões",
        subtitle_style(),
    ));
}

fn _spawn_character_creation_area(root: &mut ChildBuilder) {
    root.spawn(NodeBundle {
        style: character_creation_area_style(),
        background_color: character_creation_area_bg_color(),
        border_color: character_creation_area_border_color(),
        ..default()
    })
    .with_children(|area| {});
}

pub(super) fn cleanup_character_creation(
    mut commands: Commands,
    roots: Query<Entity, With<CharacterCreationUiRoot>>,
) {
    for entity in &roots {
        commands.entity(entity).despawn_recursive();
    }
}
