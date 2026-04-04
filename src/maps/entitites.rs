use crate::maps::components::MapConfig;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapBundle;

impl MapBundle {
    pub fn spawn(
        commands: &mut Commands,
        map_entity: Entity,
        map_size: TilemapSize,
        tile_storage: TileStorage,
        textures: &[Handle<Image>],
        config: &MapConfig,
    ) {
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
}
