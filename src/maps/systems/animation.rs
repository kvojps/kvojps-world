use crate::maps::components::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

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

pub fn animate_map_background_tiles(
	time: Res<Time>,
	active_map: Option<Res<ActiveMap>>,
	mut frame_timer: Local<Option<Timer>>,
	mut current_frame: Local<u32>,
	mut tiles: Query<(&MapBackgroundAnimatedTile, &mut TileTextureIndex)>,
) {
	let Some(active_map) = active_map else {
		return;
	};
	if active_map.config.bg_frame_paths.len() <= 1 {
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
