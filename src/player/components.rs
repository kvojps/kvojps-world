use bevy::prelude::*;

pub const SPRITE_SIZE: UVec2 = UVec2::new(40, 48);
pub const IDLE_FRAMES: usize = 4;
pub const WALK_FRAMES: usize = 4;
pub const PLAYER_SCALE: f32 = 2.0;

#[derive(Component)]
pub struct Player;

impl Player {
    pub const SPEED: f32 = 200.0;
}

#[derive(Resource)]
pub struct PlayerSetup {
    pub idle_image: Handle<Image>,
    pub walk_image: Handle<Image>,
    pub spawned: bool,
}

#[derive(Component)]
pub struct PlayerSpriteSheets {
    pub idle_image: Handle<Image>,
    pub idle_layout: Handle<TextureAtlasLayout>,
    pub walk_image: Handle<Image>,
    pub walk_layout: Handle<TextureAtlasLayout>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Down,
    Right,
    Up,
    Left,
}

impl Direction {
    pub fn row(self) -> usize {
        match self {
            Direction::Left => 0,
            Direction::Right => 1,
            Direction::Up => 2,
            Direction::Down => 3,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AnimationSet {
    Idle,
    Walk,
}

#[derive(Component)]
pub struct PlayerAnimation {
    pub direction: Direction,
    pub set: AnimationSet,
    pub frame: usize,
    pub idle_frame_count: usize,
    pub walk_frame_count: usize,
    pub idle_columns: usize,
    pub walk_columns: usize,
    pub timer: Timer,
}

impl PlayerAnimation {
    pub fn new(
        idle_columns: usize,
        walk_columns: usize,
        idle_frame_count: usize,
        walk_frame_count: usize,
    ) -> Self {
        Self {
            direction: Direction::Down,
            set: AnimationSet::Idle,
            frame: 0,
            idle_frame_count,
            walk_frame_count,
            idle_columns,
            walk_columns,
            timer: Timer::from_seconds(0.12, TimerMode::Repeating),
        }
    }

    pub fn active_columns(&self) -> usize {
        match self.set {
            AnimationSet::Idle => self.idle_columns,
            AnimationSet::Walk => self.walk_columns,
        }
    }

    pub fn active_frame_count(&self) -> usize {
        match self.set {
            AnimationSet::Idle => self.idle_frame_count,
            AnimationSet::Walk => self.walk_frame_count,
        }
    }

    pub fn atlas_index(&self) -> usize {
        self.direction.row() * self.active_columns() + self.frame
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.direction != direction {
            self.direction = direction;
            self.frame = 0;
        }
    }

    pub fn set_set(&mut self, set: AnimationSet) {
        if self.set != set {
            self.set = set;
            self.frame = 0;
            self.timer.reset();
        }
    }
}
