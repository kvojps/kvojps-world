use std::collections::HashSet;

use bevy::prelude::*;

#[derive(Resource, Clone)]
pub(super) struct OverworldMap {
    pub blocked_tiles: HashSet<IVec2>,
}

#[derive(Resource, Clone, Copy)]
pub(super) struct OverworldLayout {
    pub tile_width: f32,
    pub tile_height: f32,
}
