mod animation;
mod camera;
mod movement;
mod setup;

pub use animation::animate_player;
pub use camera::follow_player_camera;
pub use movement::movement_player;
pub use setup::{setup_player, setup_player_scene};
