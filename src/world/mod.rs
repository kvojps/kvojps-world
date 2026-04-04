use bevy::prelude::*;
pub mod systems;
use systems::setup_world_map;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world_map);
    }
}
