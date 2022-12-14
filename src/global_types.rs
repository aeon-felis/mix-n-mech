use bevy::prelude::*;
use leafwing_input_manager::Actionlike;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq)]
pub struct MenuActionForKbgp;

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum AppState {
    Menu(MenuState),
    LoadLevel,
    Game,
    LevelCompleted,
    Editor,
}

#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum MenuState {
    Main,
    LevelSelect,
    Pause,
    LevelCompleted,
    GameOver,
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
// pub enum GameSystemLabel {
// ApplyMovement,
// }

pub struct LevelProgress {
    pub just_completed: Option<String>,
    pub current_level: Option<String>,
    pub num_levels_available: usize,
}

#[derive(Actionlike, Debug, Copy, Clone, PartialEq, Eq, Hash)]
// #[allow(dead_code)]
pub enum InputBinding {
    Move,
    Pickup,
}

#[derive(Component)]
pub struct CameraInclude;

#[derive(Component)]
pub struct IsPlayer;

#[derive(Component, Default)]
pub struct Pickable {
    pub carried_by: Option<Entity>,
}

#[derive(Component, Default)]
pub struct Carrier {
    pub carrying: Option<Entity>,
}

#[derive(Component)]
pub struct IsMountBase;

#[derive(Component)]
pub struct IsPowerSource;

#[derive(Component)]
pub struct HalfHeight(pub f32);

#[derive(Component)]
pub struct Activatable {
    pub active: bool,
}

#[derive(Component, PartialEq, Eq, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HDirection {
    Left,
    Right,
}
impl HDirection {
    pub fn as_x(&self) -> f32 {
        match self {
            HDirection::Left => -1.0,
            HDirection::Right => 1.0,
        }
    }

    pub fn switch(&self) -> HDirection {
        match self {
            HDirection::Left => HDirection::Right,
            HDirection::Right => HDirection::Left,
        }
    }

    pub(crate) fn as_vec(&self) -> Vec2 {
        Vec2::new(self.as_x(), 0.0)
    }
}

#[derive(Component)]
pub struct IsDoorKey;

#[derive(Component)]
pub struct OpenableDoor {
    pub open: bool,
}
