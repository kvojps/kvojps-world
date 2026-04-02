use bevy::prelude::*;
pub mod components;
pub mod systems;

use components::Player;
use systems::player_movement;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_movement);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::new(32.0, 32.0)),
        Transform::default(),
        Player,
    ));
}
