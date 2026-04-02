use super::components::{
    AnimationSet, Direction, IDLE_FRAMES, PendingPlayerSheet, Player, PlayerAnimation,
    PlayerSpriteSheets, PLAYER_SCALE, SPRITE_SIZE, WALK_FRAMES,
};
use bevy::prelude::*;

pub fn setup_camera_and_player_sheet(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.insert_resource(PendingPlayerSheet {
        idle_image: asset_server.load("player_idle.png"),
        walk_image: asset_server.load("player_walk.png"),
        spawned: false,
    });
}

pub fn spawn_player_from_sheet_when_ready(
    mut commands: Commands,
    mut pending: ResMut<PendingPlayerSheet>,
    images: Res<Assets<Image>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if pending.spawned {
        return;
    }

    let Some(idle_image) = images.get(&pending.idle_image) else {
        return;
    };
    let Some(walk_image) = images.get(&pending.walk_image) else {
        return;
    };

    let idle_size = idle_image.texture_descriptor.size;
    let idle_columns = (idle_size.width / SPRITE_SIZE.x).max(1) as usize;
    let idle_rows = (idle_size.height / SPRITE_SIZE.y).max(1) as usize;

    let walk_size = walk_image.texture_descriptor.size;
    let walk_columns = (walk_size.width / SPRITE_SIZE.x).max(1) as usize;
    let walk_rows = (walk_size.height / SPRITE_SIZE.y).max(1) as usize;

    let idle_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        SPRITE_SIZE,
        idle_columns as u32,
        idle_rows as u32,
        None,
        None,
    ));
    let walk_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        SPRITE_SIZE,
        walk_columns as u32,
        walk_rows as u32,
        None,
        None,
    ));
    let idle_frame_count = IDLE_FRAMES.min(idle_columns);
    let walk_frame_count = WALK_FRAMES.min(walk_columns);
    let animation = PlayerAnimation::new(idle_columns, walk_columns, idle_frame_count, walk_frame_count);

    commands.spawn((
        Sprite::from_atlas_image(
            pending.idle_image.clone(),
            TextureAtlas {
                layout: idle_layout.clone(),
                index: animation.atlas_index(),
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(PLAYER_SCALE)),
        Player,
        animation,
        PlayerSpriteSheets {
            idle_image: pending.idle_image.clone(),
            idle_layout,
            walk_image: pending.walk_image.clone(),
            walk_layout,
        },
    ));

    pending.spawned = true;
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
