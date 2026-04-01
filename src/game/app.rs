use crate::game::plugin::GamePlugin;
use bevy::prelude::*;

pub fn build_app() -> App {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.04, 0.03, 0.06)));
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Kvojps World".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }),
        GamePlugin,
    ));
    app
}
