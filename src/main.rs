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
        vec![
            vec!["world/grass.png"],
            vec!["world/path.png"],
            vec![
                "world/water_row8_1.png",
                "world/water_row8_2.png",
                "world/water_row8_3.png",
                "world/water_row8_4.png",
                "world/water_row8_5.png",
                "world/water_row8_6.png",
                "world/water_row8_7.png",
                "world/water_row8_8.png",
            ],
        ],
    )
    .with_animation_frame_seconds(0.3)];

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_plugins(GamePlugin::new(PlayerPlugin, map_configs, "main"))
        .run();
}
