use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use core::panic;
use serde_json::Value;
use std::fs;

const MAP_FILE_PATH: &str = "assets/world/map.tmj";
const TILE_SIZE: f32 = 16.0;
const TILE_SCALE: f32 = 2.0;
const Z_LAYER: f32 = -1.0;

pub fn setup_world_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = _load_map();
    let grass: Handle<Image> = asset_server.load("world/grass.png");
    let path: Handle<Image> = asset_server.load("world/path.png");
    let water: Handle<Image> = asset_server.load("world/water.png");
    let terrain_textures = vec![grass, path, water];

    _spawn_main_tilemap(&mut commands, &map, terrain_textures);
}

fn _load_map() -> Map {
    let Ok(content) = fs::read_to_string(&MAP_FILE_PATH) else {
        panic!("Failed to read map file at '{}'", MAP_FILE_PATH);
    };

    let Some(map) = _parse_map(&content) else {
        panic!("Failed to parse map file at '{}'", MAP_FILE_PATH);
    };

    return map;
}

struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Vec<char>>,
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
            row.push(_map_gid_to_tile(gid));
        }
        rows.push(row);
    }

    Some(Map {
        width,
        height,
        tiles: rows,
    })
}

fn _map_gid_to_tile(gid: u32) -> char {
    match gid {
        1 => 'G',
        2 => 'P',
        3 => 'W',
        _ => panic!("Unknown tile GID: {}", gid),
    }
}

fn _spawn_main_tilemap(commands: &mut Commands, map: &Map, textures: Vec<Handle<Image>>) {
    let map_entity = commands.spawn_empty().id();
    let map_size = TilemapSize {
        x: map.width as u32,
        y: map.height as u32,
    };
    let mut tile_storage = TileStorage::empty(map_size);

    for y in 0..map.height {
        for x in 0..map.width {
            let tile = map.tiles[y][x];

            let position = TilePos {
                x: x as u32,
                y: y as u32,
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position,
                    texture_index: TileTextureIndex(_map_tile_to_gid(tile)),
                    tilemap_id: TilemapId(map_entity),
                    ..default()
                })
                .id();

            tile_storage.set(&position, tile_entity);
        }
    }

    commands.entity(map_entity).insert(TilemapBundle {
        grid_size: TilemapGridSize {
            x: TILE_SIZE,
            y: TILE_SIZE,
        },
        anchor: TilemapAnchor::Center,
        map_type: TilemapType::Square,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Vector(textures),
        tile_size: TilemapTileSize {
            x: TILE_SIZE,
            y: TILE_SIZE,
        },
        transform: Transform::from_xyz(0.0, 0.0, Z_LAYER).with_scale(Vec3::splat(TILE_SCALE)),
        ..default()
    });
}

fn _map_tile_to_gid(tile: char) -> u32 {
    match tile {
        'G' => 0,
        'P' => 1,
        'W' => 2,
        _ => panic!("Unknown tile type: {}", tile),
    }
}
