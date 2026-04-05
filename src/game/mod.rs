use bevy::prelude::*;

use crate::maps::{MapConfig, MapsPlugin};

#[derive(Clone)]
pub struct GamePlugin<P>
where
    P: Plugin + Clone,
{
    player_plugin: P,
    map_configs: Vec<MapConfig>,
    active_map_key: &'static str,
}

impl<P> GamePlugin<P>
where
    P: Plugin + Clone,
{
    pub fn new(
        player_plugin: P,
        map_configs: Vec<MapConfig>,
        active_map_key: &'static str,
    ) -> Self {
        Self {
            player_plugin,
            map_configs,
            active_map_key,
        }
    }
}

impl<P> Plugin for GamePlugin<P>
where
    P: Plugin + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MapsPlugin::with_maps(self.map_configs.clone(), self.active_map_key),
            self.player_plugin.clone(),
        ));
    }
}
