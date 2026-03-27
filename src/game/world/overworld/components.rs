use bevy::prelude::*;

#[derive(Component)]
pub(super) struct OverworldEntity;

#[derive(Component)]
pub(super) struct OverworldTile {
    pub tile_position: IVec2,
}

#[derive(Component)]
pub(super) struct PlayerGridPosition {
    pub tile_position: IVec2,
}
