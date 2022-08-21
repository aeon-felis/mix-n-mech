use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

#[derive(Clone, PartialEq)]
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
    // MoveVertical,
    // Grab,
}

#[derive(Component)]
pub struct CameraInclude;

#[derive(Component)]
pub struct IsPlayer;
