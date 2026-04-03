mod animation;
mod movement;
mod setup;

pub use animation::animate_player;
pub use movement::{follow_player_camera, movement_player};
pub use setup::{setup_player, setup_player_scene};
