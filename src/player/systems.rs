use super::components::{AnimationSet, Direction, Player, PlayerSetup, PlayerAnimation, PlayerSpriteSheets};
use bevy::prelude::*;

mod setup_player;
pub use setup_player::setup_player;

pub fn setup_camera_and_player_sheet(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.insert_resource(PlayerSetup {
        idle_image: asset_server.load("player_idle.png"),
        walk_image: asset_server.load("player_walk.png"),
        spawned: false,
    });
}

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &mut PlayerAnimation,
            &PlayerSpriteSheets,
            &mut Sprite,
        ),
        With<Player>,
    >,
) {
    let Ok((mut transform, mut animation, sheets, mut sprite)) = query.single_mut() else {
        return;
    };
    let mut input = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        input.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        input.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        input.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        input.x += 1.0;
    }

    if input.length_squared() > 0.0 {
        let movement = input.normalize();
        transform.translation += (movement * Player::SPEED * time.delta_secs()).extend(0.0);

        animation.set_set(AnimationSet::Walk);
        let direction = if movement.x.abs() > movement.y.abs() {
            if movement.x > 0.0 {
                Direction::Right
            } else {
                Direction::Left
            }
        } else if movement.y > 0.0 {
            Direction::Up
        } else {
            Direction::Down
        };

        animation.set_direction(direction);
    } else {
        animation.set_set(AnimationSet::Idle);
    }

    apply_animation_to_sprite(&animation, sheets, &mut sprite);
}

pub fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut PlayerAnimation, &PlayerSpriteSheets, &mut Sprite), With<Player>>,
) {
    let Ok((mut animation, sheets, mut sprite)) = query.single_mut() else {
        return;
    };

    let frame_count = animation.active_frame_count();
    if frame_count <= 1 {
        return;
    }

    animation.timer.tick(time.delta());
    if animation.timer.just_finished() {
        animation.frame = (animation.frame + 1) % frame_count;
        apply_animation_to_sprite(&animation, sheets, &mut sprite);
    }
}

fn apply_animation_to_sprite(
    animation: &PlayerAnimation,
    sheets: &PlayerSpriteSheets,
    sprite: &mut Sprite,
) {
    if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
        let (image, layout) = match animation.set {
            AnimationSet::Idle => (&sheets.idle_image, &sheets.idle_layout),
            AnimationSet::Walk => (&sheets.walk_image, &sheets.walk_layout),
        };
        sprite.image = image.clone();
        texture_atlas.layout = layout.clone();
        texture_atlas.index = animation.atlas_index();
    }
}
