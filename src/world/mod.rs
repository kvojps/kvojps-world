use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

pub mod systems;

const TILE_SIZE: f32 = 16.0;
const TILE_SCALE: f32 = 2.0;
const MAP_EDITOR_FILE_PATH: &str = "assets/world/map.tmj";
const MAP_FILE_PATH: &str = "assets/world/map.txt";
const FALLBACK_MAP_MATRIX: &[&str] = &[
    "WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW",
    "WGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGWWWWWWGGGGGGGGGGGGGGGGGGWWWWWWGGGGGGGGGW",
    "WGGGGGGGGGWWWWWWGGGGGGGGGGGGGGGGGGWWWWWWGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGPPGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGPPGGGGGGGGGGGGGGGGGGGGGGGW",
    "WPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPW",
    "WPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPW",
    "WGGGGGGGGGGGGGGGGGGGGGGGPPGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGPPGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGPPGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGPPGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGWWWWWWGGGGGGGGPPGGGGGGGGWWWWWWGGGGGGGGGW",
    "WGGGGGGGGGWWWWWWGGGGGGGGPPGGGGGGGGWWWWWWGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGPPGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGPPGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGPPGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGPPGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGW",
    "WGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGGW",
    "WWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWWW",
];

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world_map);
    }
}

fn setup_world_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let grass = asset_server.load("world/grass.png");
    let path = asset_server.load("world/path.png");
    let water = asset_server.load("world/water.png");
    let terrain_textures = vec![grass, path, water];
    let map_rows = load_map_rows();

    let map_height = map_rows.len();
    let map_width = map_rows.iter().map(|row| row.len()).max().unwrap_or(0);

    if map_width == 0 || map_height == 0 {
        error!("Map is empty after parsing. Tilemap will not be spawned.");
        return;
    }

    info!("Loaded map with size {}x{} tiles", map_width, map_height);

    spawn_main_tilemap(
        &mut commands,
        &map_rows,
        map_width,
        map_height,
        TILE_SIZE,
        -1.0,
        terrain_textures,
    );
}

fn spawn_main_tilemap(
    commands: &mut Commands,
    map_rows: &[Vec<char>],
    map_width: usize,
    map_height: usize,
    tile_size: f32,
    z_layer: f32,
    textures: Vec<Handle<Image>>,
) {
    let map_entity = commands.spawn_empty().id();

    let map_size = TilemapSize {
        x: map_width as u32,
        y: map_height as u32,
    };

    let mut tile_storage = TileStorage::empty(map_size);

    for y in 0..map_height {
        for x in 0..map_width {
            let tile = map_rows
                .get(y)
                .and_then(|row| row.get(x))
                .copied()
                .unwrap_or('W');

            let position = TilePos {
                x: x as u32,
                y: y as u32,
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position,
                    texture_index: TileTextureIndex(tile_texture_index(tile)),
                    tilemap_id: TilemapId(map_entity),
                    ..default()
                })
                .id();

            tile_storage.set(&position, tile_entity);
        }
    }

    commands.entity(map_entity).insert(TilemapBundle {
        grid_size: TilemapGridSize {
            x: tile_size,
            y: tile_size,
        },
        anchor: TilemapAnchor::Center,
        map_type: TilemapType::Square,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Vector(textures),
        tile_size: TilemapTileSize {
            x: tile_size,
            y: tile_size,
        },
        transform: Transform::from_xyz(0.0, 0.0, z_layer).with_scale(Vec3::splat(TILE_SCALE)),
        ..default()
    });
}

fn load_map_rows() -> Vec<Vec<char>> {
    for path in editor_map_file_candidates() {
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };

        let Some(rows) = parse_tiled_rows(&content) else {
            continue;
        };

        if !rows.is_empty() {
            info!("Loaded world map (Tiled) from {}", path.display());
            return rows;
        }
    }

    for path in text_map_file_candidates() {
        let Ok(content) = fs::read_to_string(&path) else {
            continue;
        };

        let rows = parse_map_rows(&content);
        if !rows.is_empty() {
            info!("Loaded world map from {}", path.display());
            return rows;
        }
    }

    warn!(
        "Could not load a valid map from {} or {}. Using embedded fallback map.",
        MAP_EDITOR_FILE_PATH,
        MAP_FILE_PATH
    );

    FALLBACK_MAP_MATRIX
        .iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn editor_map_file_candidates() -> Vec<PathBuf> {
    let base = Path::new(MAP_EDITOR_FILE_PATH);
    let mut candidates = vec![base.to_path_buf()];

    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd.join(base));
        candidates.push(cwd.join("..\\assets\\world\\map.tmj"));
        candidates.push(cwd.join("..\\..\\assets\\world\\map.tmj"));
    }

    candidates
}

fn text_map_file_candidates() -> Vec<PathBuf> {
    let base = Path::new(MAP_FILE_PATH);
    let mut candidates = vec![base.to_path_buf()];

    if let Ok(cwd) = std::env::current_dir() {
        candidates.push(cwd.join(base));
        candidates.push(cwd.join("..\\assets\\world\\map.txt"));
        candidates.push(cwd.join("..\\..\\assets\\world\\map.txt"));
    }

    candidates
}

fn parse_tiled_rows(content: &str) -> Option<Vec<Vec<char>>> {
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
            row.push(tile_from_tiled_gid(gid));
        }
        rows.push(row);
    }

    Some(rows)
}

fn tile_from_tiled_gid(gid: u32) -> char {
    match gid {
        1 => 'G',
        2 => 'P',
        3 => 'W',
        _ => 'W',
    }
}

fn parse_map_rows(content: &str) -> Vec<Vec<char>> {
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(parse_map_line)
        .filter(|row| !row.is_empty())
        .collect()
}

fn parse_map_line(line: &str) -> Vec<char> {
    line.chars()
        .map(|ch| ch.to_ascii_uppercase())
        .filter(|tile| !tile.is_whitespace())
        .map(|tile| match tile {
            'G' | 'P' | 'W' => tile,
            _ => 'W',
        })
        .collect()
}

fn tile_texture_index(tile: char) -> u32 {
    match tile {
        'G' => 0,
        'P' => 1,
        'W' => 2,
        _ => 2,
    }
}
