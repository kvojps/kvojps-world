use crate::maps::components::{MapBackgroundAnimatedTile, MapBackgroundTilemap, MapConfig};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ecs_tilemap::prelude::*;

pub(crate) fn spawn_map_background(
	commands: &mut Commands,
	asset_server: &AssetServer,
	windows: &Query<&Window, With<PrimaryWindow>>,
	config: &MapConfig,
) {
	if config.bg_frame_paths.is_empty() {
		return;
	}

	let textures: Vec<Handle<Image>> = config
		.bg_frame_paths
		.iter()
		.map(|path| asset_server.load(*path))
		.collect();

	let pixels_per_tile = (config.tile_size * config.tile_scale).max(1.0);
	let (tiles_x, tiles_y) = if let Ok(window) = windows.single() {
		(
			(window.width() / pixels_per_tile).ceil() as u32
				+ config.bg_viewport_padding_tiles * 2,
			(window.height() / pixels_per_tile).ceil() as u32
				+ config.bg_viewport_padding_tiles * 2,
		)
	} else {
		(64, 48)
	};

	let background_entity = commands.spawn((MapBackgroundTilemap,)).id();
	let map_size = TilemapSize {
		x: tiles_x.max(1),
		y: tiles_y.max(1),
	};
	let mut tile_storage = TileStorage::empty(map_size);

	for y in 0..map_size.y {
		for x in 0..map_size.x {
			let position = TilePos { x, y };
			let mut tile_commands = commands.spawn(TileBundle {
				position,
				texture_index: TileTextureIndex(0),
				tilemap_id: TilemapId(background_entity),
				..default()
			});

			if textures.len() > 1 {
				tile_commands.insert(MapBackgroundAnimatedTile {
					start_index: 0,
					frame_count: textures.len() as u32,
				});
			}

			let tile_entity = tile_commands.id();
			tile_storage.set(&position, tile_entity);
		}
	}

	commands.entity(background_entity).insert(TilemapBundle {
		grid_size: TilemapGridSize {
			x: config.tile_size,
			y: config.tile_size,
		},
		anchor: TilemapAnchor::Center,
		map_type: TilemapType::Square,
		size: map_size,
		storage: tile_storage,
		texture: TilemapTexture::Vector(textures),
		tile_size: TilemapTileSize {
			x: config.tile_size,
			y: config.tile_size,
		},
		transform: Transform::from_xyz(0.0, 0.0, config.bg_z_layer)
			.with_scale(Vec3::splat(config.tile_scale)),
		..default()
	});
}

pub fn sync_map_background_tilemap(
	camera_query: Query<&Transform, (With<Camera2d>, Without<MapBackgroundTilemap>)>,
	mut background_query: Query<&mut Transform, With<MapBackgroundTilemap>>,
) {
	let Ok(camera_transform) = camera_query.single() else {
		return;
	};

	for mut background_transform in &mut background_query {
		background_transform.translation.x = camera_transform.translation.x;
		background_transform.translation.y = camera_transform.translation.y;
	}
}
