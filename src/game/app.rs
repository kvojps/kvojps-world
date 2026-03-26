use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::game::plugin::GamePlugin;

pub fn build_app() -> App {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::srgb(0.04, 0.03, 0.06)));

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust RPG".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }),
        EguiPlugin,
        GamePlugin,
    ));

    app
}
