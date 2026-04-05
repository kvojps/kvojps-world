use bevy::prelude::*;

#[derive(Clone)]
pub struct MapConfig {
    pub key: &'static str,
    pub file_path: &'static str,
    pub texture_paths: Vec<Vec<&'static str>>,
    pub tile_size: f32,
    pub tile_scale: f32,
    pub z_layer: f32,
    pub animation_frame_seconds: f32,
    pub bg_frame_paths: Vec<&'static str>,
    pub bg_viewport_padding_tiles: u32,
    pub bg_z_layer: f32,
}

impl MapConfig {
    pub fn new(
        key: &'static str,
        file_path: &'static str,
        texture_paths: Vec<Vec<&'static str>>,
    ) -> Self {
        Self {
            key,
            file_path,
            texture_paths,
            tile_size: 16.0,
            tile_scale: 2.0,
            z_layer: -1.0,
            animation_frame_seconds: 0.3,
            bg_frame_paths: Vec::new(),
            bg_viewport_padding_tiles: 4,
            bg_z_layer: -2.0,
        }
    }

    pub fn with_bg_frames(mut self, frame_paths: Vec<&'static str>) -> Self {
        self.bg_frame_paths = frame_paths;
        self
    }
}

#[derive(Resource, Clone)]
pub struct MapCatalog(pub Vec<MapConfig>);

#[derive(Resource, Clone)]
pub struct ActiveMapKey(pub &'static str);

#[derive(Resource, Clone)]
pub struct ActiveMap {
    pub config: MapConfig,
}

#[derive(Component)]
pub struct MapAnimatedTile {
    pub start_index: u32,
    pub frame_count: u32,
}

#[derive(Component)]
pub struct MapBackgroundTilemap;

#[derive(Component)]
pub struct MapBackgroundAnimatedTile {
    pub start_index: u32,
    pub frame_count: u32,
}
