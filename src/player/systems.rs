use super::components::{
    AnimationSet, Direction, IDLE_FRAMES, PLAYER_SCALE, Player, PlayerAnimation, PlayerSetup,
    PlayerSpriteSheets, SPRITE_SIZE, WALK_FRAMES,
};
use bevy::prelude::*;

pub fn setup_camera_and_player_sheet(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.insert_resource(PlayerSetup {
        idle_image: asset_server.load("player_idle.png"),
        walk_image: asset_server.load("player_walk.png"),
        spawned: false,
    });
}

pub fn setup_player(
    mut commands: Commands,
    mut player_setup: ResMut<PlayerSetup>,
    images: Res<Assets<Image>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // This is a fallback in case the player wasn't spawned during the startup phase.
    if player_setup.spawned {
        return;
    }
    let Some(idle_image) = images.get(&player_setup.idle_image) else {
        return;
    };
    let Some(walk_image) = images.get(&player_setup.walk_image) else {
        return;
    };

    // Calculate layout and frame info for both idle and walk sheets
    let idle_layout = _get_layout(idle_image, IDLE_FRAMES, &mut layouts);
    let walk_layout = _get_layout(walk_image, WALK_FRAMES, &mut layouts);
    let animation = PlayerAnimation::new(
        idle_layout.columns,
        walk_layout.columns,
        idle_layout.frame_count,
        walk_layout.frame_count,
    );

    // Spawn the player entity with the idle sprite and animation components
    commands.spawn((
        Sprite::from_atlas_image(
            player_setup.idle_image.clone(),
            TextureAtlas {
                layout: idle_layout.layout.clone(),
                index: animation.atlas_index(),
            },
        ),
        Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(PLAYER_SCALE)),
        Player,
        animation,
        PlayerSpriteSheets {
            idle_image: player_setup.idle_image.clone(),
            idle_layout: idle_layout.layout.clone(),
            walk_image: player_setup.walk_image.clone(),
            walk_layout: walk_layout.layout.clone(),
        },
    ));

    player_setup.spawned = true;
}

pub fn _get_layout(
    image: &Image,
    frames: usize,
    layouts: &mut Assets<TextureAtlasLayout>,
) -> AtlasLayoutInfo {
    let size = image.texture_descriptor.size;
    let columns = (size.width / SPRITE_SIZE.x).max(1) as usize;
    let rows = (size.height / SPRITE_SIZE.y).max(1) as usize;
    let frame_count = frames.min(columns);

    let layout = layouts.add(TextureAtlasLayout::from_grid(
        SPRITE_SIZE,
        columns as u32,
        rows as u32,
        None,
        None,
    ));

    AtlasLayoutInfo {
        columns,
        frame_count,
        layout,
    }
}

pub struct AtlasLayoutInfo {
    pub columns: usize,
    pub frame_count: usize,
    pub layout: Handle<TextureAtlasLayout>,
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
