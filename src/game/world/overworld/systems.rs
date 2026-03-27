use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::EguiContexts;
use bevy_egui::egui::{self, Align, Color32, Layout, RichText};

use crate::game::gameplay::player::Player;
use crate::game::states::AppScreen;

use super::components::{OverworldTile, PlayerGridPosition};
use super::map::{MAP_HEIGHT, MAP_WIDTH, tile_inside_bounds, tile_to_world};
use super::state::{OverworldLayout, OverworldMap};

pub(super) fn return_to_menu(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_screen: ResMut<NextState<AppScreen>>,
    mut clear_color: ResMut<ClearColor>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        clear_color.0 = Color::srgb(0.04, 0.03, 0.06);
        next_screen.set(AppScreen::MainMenu);
    }
}

pub(super) fn handle_overworld_player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    map: Res<OverworldMap>,
    layout: Res<OverworldLayout>,
    mut player_query: Query<(&mut PlayerGridPosition, &mut Transform), With<Player>>,
) {
    let direction = if keyboard.just_pressed(KeyCode::ArrowUp) || keyboard.just_pressed(KeyCode::KeyW)
    {
        Some(IVec2::new(0, 1))
    } else if keyboard.just_pressed(KeyCode::ArrowDown) || keyboard.just_pressed(KeyCode::KeyS) {
        Some(IVec2::new(0, -1))
    } else if keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::KeyA) {
        Some(IVec2::new(-1, 0))
    } else if keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::KeyD) {
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

pub(super) fn sync_overworld_layout(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut layout: ResMut<OverworldLayout>,
    mut spatial_queries: ParamSet<(
        Query<(&OverworldTile, &mut Transform, &mut Sprite)>,
        Query<(&PlayerGridPosition, &mut Transform, &mut Sprite), With<Player>>,
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

pub(super) fn draw_overworld_overlay(
    mut egui_contexts: EguiContexts,
    player_query: Query<(&Player, &PlayerGridPosition)>,
) {
    let (player_name, class_name, details_label, tile_label) = player_query
        .get_single()
        .map(|(player, grid)| {
            (
                player.name.clone(),
                player.class.label().to_string(),
                format!(
                    "{} | Nv. {} | HP {}/{}",
                    player.gender.label(),
                    player.level,
                    player.hp_current,
                    player.hp_max
                ),
                format!("Tile: {}, {}", grid.tile_position.x, grid.tile_position.y),
            )
        })
        .unwrap_or_else(|_| {
            (
                "Heroi".to_string(),
                "Guerreiro".to_string(),
                "Masculino | Nv. 1 | HP 100/100".to_string(),
                "Tile: ?".to_string(),
            )
        });

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
                        ui.label(
                            RichText::new(details_label)
                                .size(13.0)
                                .color(Color32::from_rgb(171, 189, 214)),
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
