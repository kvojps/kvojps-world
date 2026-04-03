use super::animation::apply_animation_to_sprite;
use crate::player::components::{
    AnimationSet, Direction, Player, PlayerAnimation, PlayerSpriteSheets,
};
use bevy::prelude::*;

pub fn movement_player(
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

    let has_movement = input.length_squared() > 0.0;
    if has_movement {
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

pub fn follow_player_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}
