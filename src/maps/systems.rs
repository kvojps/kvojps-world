use crate::maps::{ActiveMap, ActiveMapKey, MapCatalog, MapConfig};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde_json::Value;
use std::fs;

pub fn select_active_map(
    mut commands: Commands,
    map_catalog: Res<MapCatalog>,
    active_map_key: Res<ActiveMapKey>,
) {
    let Some(config) = map_catalog
        .0
        .iter()
        .find(|candidate| candidate.key == active_map_key.0)
        .cloned()
    else {
        panic!(
            "Active map key '{}' was not found in configured maps",
            active_map_key.0
        );
    };

    commands.insert_resource(ActiveMap { config });
}

pub fn setup_map_from_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    active_map: Res<ActiveMap>,
) {
    let map = load_map(active_map.config.file_path);
    let textures: Vec<Handle<Image>> = active_map
        .config
        .texture_paths
        .iter()
        .map(|path| asset_server.load(*path))
        .collect();

    spawn_tilemap(&mut commands, &map, &textures, &active_map.config);
}

fn load_map(map_file_path: &str) -> Map {
    let Ok(content) = fs::read_to_string(map_file_path) else {
        panic!("Failed to read map file at '{}'", map_file_path);
    };

    let Some(map) = parse_map(&content) else {
        panic!("Failed to parse map file at '{}'", map_file_path);
    };

    map
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Option<u32>>>,
}

fn parse_map(content: &str) -> Option<Map> {
    let root: Value = serde_json::from_str(content).ok()?;
    let width = root.get("width")?.as_u64()? as usize;
    let height = root.get("height")?.as_u64()? as usize;
    if width == 0 || height == 0 {
        return None;
    }

    let layers = root.get("layers")?.as_array()?;
    let tile_layer = layers
        .iter()
        .find(|layer| layer.get("type").and_then(Value::as_str) == Some("tilelayer"))?;
    let data = tile_layer.get("data")?.as_array()?;
    if data.len() != width * height {
        return None;
    }

    let mut rows = Vec::with_capacity(height);
    for y in 0..height {
        let mut row = Vec::with_capacity(width);
        for x in 0..width {
            let raw_gid = data[y * width + x].as_u64().unwrap_or(0) as u32;
            let gid = raw_gid & 0x1FFF_FFFF;

            if gid == 0 {
                row.push(None);
            } else {
                row.push(Some(gid - 1));
            }
        }
        rows.push(row);
    }

    Some(Map {
        width,
        height,
        tiles: rows,
    })
}

fn spawn_tilemap(commands: &mut Commands, map: &Map, textures: &[Handle<Image>], config: &MapConfig) {
    let map_entity = commands.spawn_empty().id();
    let map_size = TilemapSize {
        x: map.width as u32,
        y: map.height as u32,
    };
    let mut tile_storage = TileStorage::empty(map_size);

    for y in 0..map.height {
        for x in 0..map.width {
            let Some(texture_index) = map.tiles[y][x] else {
                continue;
            };

            if texture_index as usize >= textures.len() {
                panic!(
                    "Tile references texture index {} but only {} textures were provided",
                    texture_index,
                    textures.len()
                );
            }

            let position = TilePos {
                x: x as u32,
                y: y as u32,
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position,
                    texture_index: TileTextureIndex(texture_index),
                    tilemap_id: TilemapId(map_entity),
                    ..default()
                })
                .id();

            tile_storage.set(&position, tile_entity);
        }
    }

    commands.entity(map_entity).insert(TilemapBundle {
        grid_size: TilemapGridSize {
            x: config.tile_size,
            y: config.tile_size,
        },
        anchor: TilemapAnchor::Center,
        map_type: TilemapType::Square,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Vector(textures.to_vec()),
        tile_size: TilemapTileSize {
            x: config.tile_size,
            y: config.tile_size,
        },
        transform: Transform::from_xyz(0.0, 0.0, config.z_layer)
            .with_scale(Vec3::splat(config.tile_scale)),
        ..default()
    });
}
