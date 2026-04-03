use crate::player::components::{AnimationSet, PlayerAnimation, PlayerSpriteSheets};
use bevy::prelude::*;

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
