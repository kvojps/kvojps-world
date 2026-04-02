use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

impl Player {
    pub const SPEED: f32 = 200.0;
}
