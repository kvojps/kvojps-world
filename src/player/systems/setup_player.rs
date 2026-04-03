use crate::player::components::{
    IDLE_FRAMES, PLAYER_SCALE, Player, PlayerAnimation, PlayerSetup, PlayerSpriteSheets,
    SPRITE_SIZE, WALK_FRAMES,
};
use bevy::prelude::*;

pub fn setup_player(
    mut commands: Commands,
    mut player_setup: ResMut<PlayerSetup>,
    images: Res<Assets<Image>>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if player_setup.spawned {
        return;
    }

    let Some(idle_image) = images.get(&player_setup.idle_image) else {
        return;
    };
    let Some(walk_image) = images.get(&player_setup.walk_image) else {
        return;
    };

    let idle_layout = _get_layout(idle_image, IDLE_FRAMES, &mut layouts);
    let walk_layout = _get_layout(walk_image, WALK_FRAMES, &mut layouts);
    let animation = PlayerAnimation::new(
        idle_layout.columns,
        walk_layout.columns,
        idle_layout.frame_count,
        walk_layout.frame_count,
    );

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
            idle_layout: idle_layout.layout,
            walk_image: player_setup.walk_image.clone(),
            walk_layout: walk_layout.layout,
        },
    ));

    player_setup.spawned = true;
}

struct AtlasLayoutInfo {
    columns: usize,
    frame_count: usize,
    layout: Handle<TextureAtlasLayout>,
}

fn _get_layout(
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
