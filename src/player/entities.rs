use crate::player::components::{PLAYER_SCALE, Player, PlayerAnimation, PlayerSpriteSheets};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite: Sprite,
    transform: Transform,
    player: Player,
    sprite_sheets: PlayerSpriteSheets,
    animation: PlayerAnimation,
}

impl PlayerBundle {
    pub fn new(
        idle_image: Handle<Image>,
        idle_layout: Handle<TextureAtlasLayout>,
        walk_image: Handle<Image>,
        walk_layout: Handle<TextureAtlasLayout>,
        animation: PlayerAnimation,
    ) -> Self {
        Self {
            sprite: Sprite::from_atlas_image(
                idle_image.clone(),
                TextureAtlas {
                    layout: idle_layout.clone(),
                    index: animation.atlas_index(),
                },
            ),
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(PLAYER_SCALE)),
            player: Player,
            sprite_sheets: PlayerSpriteSheets {
                idle_image,
                idle_layout,
                walk_image,
                walk_layout,
            },
            animation,
        }
    }
}
