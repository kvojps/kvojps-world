use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
mod player;
mod world;
use player::PlayerPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
