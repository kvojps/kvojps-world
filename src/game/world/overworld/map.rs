use std::collections::HashSet;

use bevy::prelude::*;

use super::state::{OverworldLayout, OverworldMap};

pub(super) const MAP_WIDTH: i32 = 26;
pub(super) const MAP_HEIGHT: i32 = 16;

pub(super) fn build_overworld_map() -> OverworldMap {
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

pub(super) fn tile_inside_bounds(tile: IVec2) -> bool {
    tile.x >= 0 && tile.y >= 0 && tile.x < MAP_WIDTH && tile.y < MAP_HEIGHT
}

pub(super) fn tile_to_world(tile: IVec2, z: f32, layout: OverworldLayout) -> Vec3 {
    let world_x =
        (tile.x as f32 - MAP_WIDTH as f32 * 0.5) * layout.tile_width + layout.tile_width * 0.5;
    let world_y =
        (tile.y as f32 - MAP_HEIGHT as f32 * 0.5) * layout.tile_height + layout.tile_height * 0.5;
    Vec3::new(world_x, world_y, z)
}
