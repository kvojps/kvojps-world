use crate::maps::components::*;
use crate::maps::entitites::MapBundle;
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

pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    active_map: Res<ActiveMap>,
) {
    let map = _load_map(active_map.config.file_path);
    let mut textures: Vec<Handle<Image>> = Vec::new();
    for (group_index, texture_group) in active_map.config.texture_paths.iter().enumerate() {
        if texture_group.is_empty() {
            panic!("Texture group at index {} is empty", group_index);
        }

        for texture_path in texture_group {
            textures.push(asset_server.load(*texture_path));
        }
    }

    _spawn_tilemap(&mut commands, &map, &textures, &active_map.config);
}

pub fn animate_map_tiles(
    time: Res<Time>,
    active_map: Option<Res<ActiveMap>>,
    mut frame_timer: Local<Option<Timer>>,
    mut current_frame: Local<u32>,
    mut tiles: Query<(&MapAnimatedTile, &mut TileTextureIndex)>,
) {
    let Some(active_map) = active_map else {
        return;
    };
    let has_animated_tiles = active_map
        .config
        .texture_paths
        .iter()
        .any(|group| group.len() > 1);
    if !has_animated_tiles {
        return;
    }

    let timer = frame_timer.get_or_insert_with(|| {
        Timer::from_seconds(
            active_map.config.animation_frame_seconds.max(0.01),
            TimerMode::Repeating,
        )
    });
    timer.tick(time.delta());

    if !timer.just_finished() {
        return;
    }

    *current_frame += 1;
    for (animated, mut texture_index) in &mut tiles {
        let frame_offset = *current_frame % animated.frame_count;
        *texture_index = TileTextureIndex(animated.start_index + frame_offset);
    }
}

fn _load_map(map_file_path: &str) -> Map {
    let Ok(content) = fs::read_to_string(map_file_path) else {
        panic!("Failed to read map file at '{}'", map_file_path);
    };

    let Some(map) = _parse_map(&content) else {
        panic!("Failed to parse map file at '{}'", map_file_path);
    };

    map
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Option<u32>>>,
}

fn _parse_map(content: &str) -> Option<Map> {
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

fn _spawn_tilemap(
    commands: &mut Commands,
    map: &Map,
    textures: &[Handle<Image>],
    config: &MapConfig,
) {
    let mut texture_offsets: Vec<u32> = Vec::with_capacity(config.texture_paths.len());
    let mut running_texture_index = 0u32;
    for texture_group in &config.texture_paths {
        texture_offsets.push(running_texture_index);
        running_texture_index += texture_group.len() as u32;
    }

    let map_entity = commands.spawn_empty().id();
    let map_size = TilemapSize {
        x: map.width as u32,
        y: map.height as u32,
    };
    let mut tile_storage = TileStorage::empty(map_size);

    for y in 0..map.height {
        for x in 0..map.width {
            let Some(logical_texture_index) = map.tiles[y][x] else {
                continue;
            };
            if logical_texture_index as usize >= config.texture_paths.len() {
                panic!(
                    "Tile references logical texture index {} but only {} texture groups were provided",
                    logical_texture_index,
                    config.texture_paths.len()
                );
            }

            let texture_group = &config.texture_paths[logical_texture_index as usize];
            let mapped_texture_index = texture_offsets[logical_texture_index as usize];
            if mapped_texture_index as usize >= textures.len() {
                panic!(
                    "Mapped texture index {} is out of bounds for {} loaded textures",
                    mapped_texture_index,
                    textures.len()
                );
            }

            let position = TilePos {
                x: x as u32,
                y: y as u32,
            };
            let mut tile_commands = commands.spawn(TileBundle {
                    position,
                    texture_index: TileTextureIndex(mapped_texture_index),
                    tilemap_id: TilemapId(map_entity),
                    ..default()
                });

            if texture_group.len() > 1 {
                tile_commands.insert(MapAnimatedTile {
                    start_index: mapped_texture_index,
                    frame_count: texture_group.len() as u32,
                });
            }

            let tile_entity = tile_commands.id();

            tile_storage.set(&position, tile_entity);
        }
    }

    MapBundle::spawn(
        commands,
        map_entity,
        map_size,
        tile_storage,
        textures,
        config,
    );
}
