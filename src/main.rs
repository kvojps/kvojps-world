use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
mod game;
mod maps;
mod player;
use game::GamePlugin;
use maps::MapConfig;
use player::PlayerPlugin;

fn main() {
    let map_configs = vec![
        MapConfig::new(
            "overworld",
            "assets/maps/overworld.tmj",
            vec![vec!["ground/grass.png"], vec!["ground/path.png"]],
        )
        .with_bg_frames(vec![
            "ocean/frame_0.png",
            "ocean/frame_1.png",
            "ocean/frame_2.png",
            "ocean/frame_3.png",
            "ocean/frame_4.png",
            "ocean/frame_5.png",
            "ocean/frame_6.png",
            "ocean/frame_7.png",
        ]),
    ];

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_plugins(GamePlugin::new(PlayerPlugin, map_configs, "overworld"))
        .run();
}
