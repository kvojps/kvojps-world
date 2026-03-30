use bevy::app::AppExit;
use bevy::prelude::*;

use crate::game::character_creation::CharacterCreationPlugin;
use crate::game::main_menu::MainMenuPlugin;
use crate::game::states::AppScreen;
use crate::game::world::overworld::OverworldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppScreen>()
            .add_plugins((MainMenuPlugin, CharacterCreationPlugin, OverworldPlugin))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, close_on_alt_f4);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn close_on_alt_f4(keyboard: Res<ButtonInput<KeyCode>>, mut app_exit_events: EventWriter<AppExit>) {
    if keyboard.pressed(KeyCode::AltLeft) && keyboard.just_pressed(KeyCode::F4) {
        app_exit_events.send(AppExit::Success);
    }
}
