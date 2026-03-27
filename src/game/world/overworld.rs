use std::collections::HashSet;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::{self, Align, Color32, Layout, RichText};
use bevy_egui::EguiContexts;

use crate::game::states::AppScreen;
use crate::game::ui::character_creation::CharacterCreationState;

const MAP_WIDTH: i32 = 26;
const MAP_HEIGHT: i32 = 16;

#[derive(Resource, Clone)]
struct OverworldMap {
    blocked_tiles: HashSet<IVec2>,
}

#[derive(Resource, Clone, Copy)]
struct OverworldLayout {
    tile_width: f32,
    tile_height: f32,
}

#[derive(Component)]
struct OverworldEntity;

#[derive(Component)]
struct OverworldTile {
    tile_position: IVec2,
}

#[derive(Component)]
struct OverworldPlayer {
    tile_position: IVec2,
}

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppScreen::Overworld), setup_overworld)
            .add_systems(
                Update,
                (
                    return_to_menu,
                    handle_overworld_player_movement,
                    sync_overworld_layout,
                    draw_overworld_overlay,
                )
                    .run_if(in_state(AppScreen::Overworld)),
            )
            .add_systems(OnExit(AppScreen::Overworld), cleanup_overworld);
    }
}

fn setup_overworld(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<OverworldPlayer>)>,
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
        OverworldPlayer {
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

fn return_to_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_screen: ResMut<NextState<AppScreen>>,
    mut clear_color: ResMut<ClearColor>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        clear_color.0 = Color::srgb(0.04, 0.03, 0.06);
        next_screen.set(AppScreen::MainMenu);
    }
}

fn handle_overworld_player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    map: Res<OverworldMap>,
    layout: Res<OverworldLayout>,
    mut player_query: Query<(&mut OverworldPlayer, &mut Transform)>,
) {
    let direction = if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW)
    {
        Some(IVec2::new(0, 1))
    } else if keyboard.just_pressed(KeyCode::ArrowDown)
        || keyboard.just_pressed(KeyCode::KeyS)
    {
        Some(IVec2::new(0, -1))
    } else if keyboard.just_pressed(KeyCode::ArrowLeft)
        || keyboard.just_pressed(KeyCode::KeyA)
    {
        Some(IVec2::new(-1, 0))
    } else if keyboard.just_pressed(KeyCode::ArrowRight)
        || keyboard.just_pressed(KeyCode::KeyD)
    {
        Some(IVec2::new(1, 0))
    } else {
        None
    };

    let Some(direction) = direction else {
        return;
    };

    let Ok((mut player, mut player_transform)) = player_query.get_single_mut() else {
        return;
    };

    let next_tile = player.tile_position + direction;
    if !tile_inside_bounds(next_tile) || map.blocked_tiles.contains(&next_tile) {
        return;
    }

    player.tile_position = next_tile;
    player_transform.translation = tile_to_world(next_tile, player_transform.translation.z, *layout);
}

fn sync_overworld_layout(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut layout: ResMut<OverworldLayout>,
    mut spatial_queries: ParamSet<(
        Query<(&OverworldTile, &mut Transform, &mut Sprite)>,
        Query<(&OverworldPlayer, &mut Transform, &mut Sprite)>,
    )>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    let new_layout = OverworldLayout {
        tile_width: window.resolution.width() / MAP_WIDTH as f32,
        tile_height: window.resolution.height() / MAP_HEIGHT as f32,
    };

    if (new_layout.tile_width - layout.tile_width).abs() < f32::EPSILON
        && (new_layout.tile_height - layout.tile_height).abs() < f32::EPSILON
    {
        return;
    }

    *layout = new_layout;

    for (tile, mut transform, mut sprite) in &mut spatial_queries.p0() {
        transform.translation = tile_to_world(tile.tile_position, transform.translation.z, new_layout);
        sprite.custom_size = Some(Vec2::new(new_layout.tile_width, new_layout.tile_height));
    }

    if let Ok((player, mut transform, mut sprite)) = spatial_queries.p1().get_single_mut() {
        transform.translation = tile_to_world(player.tile_position, transform.translation.z, new_layout);
        sprite.custom_size = Some(Vec2::splat(
            new_layout.tile_width.min(new_layout.tile_height) * 0.72,
        ));
    }
}

fn draw_overworld_overlay(
    mut egui_contexts: EguiContexts,
    player_query: Query<&OverworldPlayer>,
    creation_state: Option<Res<CharacterCreationState>>,
) {
    let player_name = creation_state
        .as_ref()
        .map_or("Heroi", |state| state.character_name.trim())
        .to_string();
    let selected_class = creation_state
        .as_ref()
        .map(|state| state.selected_class)
        .unwrap_or(0);
    let class_name = match selected_class {
        0 => "Guerreiro",
        1 => "Ranger",
        2 => "Mago",
        _ => "Clerigo",
    };
    let tile_label = player_query
        .get_single()
        .map(|player| format!("Tile: {}, {}", player.tile_position.x, player.tile_position.y))
        .unwrap_or_else(|_| "Tile: ?".to_string());

    egui::TopBottomPanel::top("hud_top").show(egui_contexts.ctx_mut(), |ui| {
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Asterfall Frontier")
                    .size(20.0)
                    .strong()
                    .color(Color32::from_rgb(224, 206, 145)),
            );

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    RichText::new("ESC para voltar ao menu")
                        .size(14.0)
                        .color(Color32::from_rgb(150, 160, 182)),
                );
                ui.label(
                    RichText::new(tile_label)
                        .size(14.0)
                        .color(Color32::from_rgb(170, 191, 206)),
                );
            });
        });
        ui.add_space(6.0);
    });

    egui::Area::new("overworld_tip".into())
        .anchor(egui::Align2::LEFT_BOTTOM, [20.0, -20.0])
        .show(egui_contexts.ctx_mut(), |ui| {
            egui::Frame::none()
                .fill(Color32::from_rgba_unmultiplied(18, 24, 35, 212))
                .stroke(egui::Stroke::new(1.2, Color32::from_rgb(98, 124, 160)))
                .rounding(egui::Rounding::same(8.0))
                .inner_margin(egui::Margin::same(16.0))
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!("{} - {}", player_name, class_name))
                                .size(18.0)
                                .color(Color32::from_rgb(209, 220, 241)),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new("Mova com WASD ou setas. Paredes bloqueiam caminho.")
                                .size(14.0)
                                .color(Color32::from_rgb(152, 172, 200)),
                        );
                    });
                });
        });
}

fn cleanup_overworld(
    mut commands: Commands,
    overworld_entities: Query<Entity, With<OverworldEntity>>,
) {
    for entity in &overworld_entities {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<OverworldMap>();
    commands.remove_resource::<OverworldLayout>();
}

fn build_overworld_map() -> OverworldMap {
    let mut blocked_tiles = HashSet::new();

    for x in 0..MAP_WIDTH {
        blocked_tiles.insert(IVec2::new(x, 0));
        blocked_tiles.insert(IVec2::new(x, MAP_HEIGHT - 1));
    }

    for y in 0..MAP_HEIGHT {
        blocked_tiles.insert(IVec2::new(0, y));
        blocked_tiles.insert(IVec2::new(MAP_WIDTH - 1, y));
    }

    for x in 5..(MAP_WIDTH - 4) {
        if x != 11 && x != 12 {
            blocked_tiles.insert(IVec2::new(x, 7));
        }
    }

    for y in 3..12 {
        if y != 8 {
            blocked_tiles.insert(IVec2::new(16, y));
        }
    }

    OverworldMap { blocked_tiles }
}

fn tile_inside_bounds(tile: IVec2) -> bool {
    tile.x >= 0 && tile.y >= 0 && tile.x < MAP_WIDTH && tile.y < MAP_HEIGHT
}

fn tile_to_world(tile: IVec2, z: f32, layout: OverworldLayout) -> Vec3 {
    let world_x =
        (tile.x as f32 - MAP_WIDTH as f32 * 0.5) * layout.tile_width + layout.tile_width * 0.5;
    let world_y =
        (tile.y as f32 - MAP_HEIGHT as f32 * 0.5) * layout.tile_height + layout.tile_height * 0.5;
    Vec3::new(world_x, world_y, z)
}
