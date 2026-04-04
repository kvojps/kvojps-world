use bevy::prelude::*;

#[derive(Clone)]
pub struct MapConfig {
    pub key: &'static str,
    pub file_path: &'static str,
    pub texture_paths: Vec<&'static str>,
    pub tile_size: f32,
    pub tile_scale: f32,
    pub z_layer: f32,
}

impl MapConfig {
    pub fn new(
        key: &'static str,
        file_path: &'static str,
        texture_paths: Vec<&'static str>,
    ) -> Self {
        Self {
            key,
            file_path,
            texture_paths,
            tile_size: 16.0,
            tile_scale: 2.0,
            z_layer: -1.0,
        }
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
