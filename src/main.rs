use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
mod game;
mod maps;
mod player;
use game::GamePlugin;
use maps::MapConfig;
use player::PlayerPlugin;

fn main() {
    let map_configs = vec![MapConfig::new(
        "main",
        "assets/world/map.tmj",
        vec!["world/grass.png", "world/path.png", "world/water.png"],
    )];

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_plugins(GamePlugin::new(PlayerPlugin, map_configs, "main"))
        .run();
}
