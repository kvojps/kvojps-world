use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::gameplay::player::Player;
use crate::game::ui::character_creation::CharacterCreationState;

use super::components::{OverworldEntity, OverworldTile, PlayerGridPosition};
use super::map::{MAP_HEIGHT, MAP_WIDTH, build_overworld_map, tile_to_world};
use super::state::OverworldLayout;

pub(super) fn setup_overworld(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<PlayerGridPosition>)>,
    creation_state: Option<Res<CharacterCreationState>>,
) {
    clear_color.0 = Color::srgb(0.03, 0.05, 0.08);
    let map = build_overworld_map();
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let layout = OverworldLayout {
        tile_width: window.resolution.width() / MAP_WIDTH as f32,
        tile_height: window.resolution.height() / MAP_HEIGHT as f32,
    };

    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let tile = IVec2::new(x, y);
            let is_blocked = map.blocked_tiles.contains(&tile);
            let tile_color = if is_blocked {
                Color::srgb(0.13, 0.17, 0.21)
            } else {
                Color::srgb(0.10, 0.24, 0.19)
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: tile_color,
                        custom_size: Some(Vec2::new(layout.tile_width, layout.tile_height)),
                        ..default()
                    },
                    transform: Transform::from_translation(tile_to_world(tile, 0.0, layout)),
                    ..default()
                },
                OverworldTile {
                    tile_position: tile,
                },
                OverworldEntity,
            ));
        }
    }

    let spawn_tile = IVec2::new(2, 2);
    let player_data = creation_state
        .as_deref()
        .map(Player::from_creation_state)
        .unwrap_or_else(Player::fallback);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.92, 0.83, 0.37),
                custom_size: Some(Vec2::splat(layout.tile_width.min(layout.tile_height) * 0.72)),
                ..default()
            },
            transform: Transform::from_translation(tile_to_world(spawn_tile, 2.0, layout)),
            ..default()
        },
        player_data,
        PlayerGridPosition {
            tile_position: spawn_tile,
        },
        OverworldEntity,
    ));

    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        camera_transform.translation.x = 0.0;
        camera_transform.translation.y = 0.0;
    }

    commands.insert_resource(map);
    commands.insert_resource(layout);
}

pub(super) fn cleanup_overworld(
    mut commands: Commands,
    overworld_entities: Query<Entity, With<OverworldEntity>>,
) {
    for entity in &overworld_entities {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<super::state::OverworldMap>();
    commands.remove_resource::<OverworldLayout>();
}
