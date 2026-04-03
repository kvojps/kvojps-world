use bevy::prelude::*;

const TILE_SIZE: f32 = 16.0;
const TILE_SCALE: f32 = 2.0;
const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 80;

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

    let width = MAP_WIDTH as f32;
    let height = MAP_HEIGHT as f32;
    let tile_world_size = TILE_SIZE * TILE_SCALE;

    let x_offset = -((width - 1.0) * tile_world_size) * 0.5;
    let y_offset = ((height - 1.0) * tile_world_size) * 0.5;

    for row in 0..MAP_HEIGHT {
        for col in 0..MAP_WIDTH {
            let texture = match tile_kind(row, col) {
                TileKind::Grass => grass.clone(),
                TileKind::Path => path.clone(),
                TileKind::Water => water.clone(),
            };

            commands.spawn((
                Sprite::from_image(texture),
                Transform::from_xyz(
                    x_offset + col as f32 * tile_world_size,
                    y_offset - row as f32 * tile_world_size,
                    -1.0,
                )
                .with_scale(Vec3::splat(TILE_SCALE)),
            ));
        }
    }
}

#[derive(Clone, Copy)]
enum TileKind {
    Grass,
    Path,
    Water,
}

fn tile_kind(row: usize, col: usize) -> TileKind {
    let is_border = row == 0 || col == 0 || row == MAP_HEIGHT - 1 || col == MAP_WIDTH - 1;
    if is_border {
        return TileKind::Water;
    }

    let center_row = MAP_HEIGHT / 2;
    let center_col = MAP_WIDTH / 2;

    let horizontal_road = row.abs_diff(center_row) <= 1;
    let vertical_road = col.abs_diff(center_col) <= 1;

    if horizontal_road || vertical_road {
        return TileKind::Path;
    }

    let lake_band = row > MAP_HEIGHT / 4 && row < MAP_HEIGHT / 3 && col % 9 <= 1;
    if lake_band {
        return TileKind::Water;
    }

    TileKind::Grass
}
