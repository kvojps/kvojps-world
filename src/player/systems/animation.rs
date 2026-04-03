use crate::player::components::{AnimationSet, Player, PlayerAnimation, PlayerSpriteSheets};
use bevy::prelude::*;

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

pub fn apply_animation_to_sprite(
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
