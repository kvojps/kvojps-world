use bevy::prelude::*;

pub mod components;
pub mod entitites;
pub mod systems;

pub use components::MapConfig;

use components::{ActiveMapKey, MapCatalog};
use self::systems::{
    animate_map_background_tiles, animate_map_tiles, select_active_map, setup_map,
    setup_map_background,
    sync_map_background_tilemap,
};

#[derive(Clone)]
pub struct MapsPlugin {
    configs: Vec<MapConfig>,
    active_map_key: &'static str,
}

impl MapsPlugin {
    pub fn with_maps(configs: Vec<MapConfig>, active_map_key: &'static str) -> Self {
        Self {
            configs,
            active_map_key,
        }
    }
}

impl Plugin for MapsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapCatalog(self.configs.clone()))
            .insert_resource(ActiveMapKey(self.active_map_key))
            .add_systems(
                Startup,
                (select_active_map, setup_map_background, setup_map).chain(),
            )
            .add_systems(Update, (animate_map_tiles, animate_map_background_tiles))
            .add_systems(PostUpdate, sync_map_background_tilemap);
    }
}
