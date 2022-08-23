mod camera;
mod global_types;
mod level_progress;
mod loading;
mod menu;
mod parts_manipulation;
mod physics_utils;
mod player;
mod player_control;
mod robot_part;
mod utils;
mod wall;
mod yoleck_utils;

use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;
use bevy_yoleck::{YoleckLoadingCommand, YoleckManaged, YoleckSyncWithEditorState};

use self::camera::CameraPlugin;
use self::global_types::{AppState, LevelProgress, MenuState};
use self::level_progress::LevelProgressPlugin;
use self::loading::LoadingPlugin;
use self::menu::MenuPlugin;
use self::parts_manipulation::PartsManipulationPlugin;
use self::player::PlayerPlugin;
use self::player_control::PlayerControlPlugin;
use self::robot_part::RobotPartPlugin;
use self::wall::WallPlugin;

pub use self::global_types::MenuActionForKbgp;

pub struct GamePlugin {
    pub is_editor: bool,
    pub start_at_level: Option<String>,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(LoadingPlugin);
        app.add_plugin(CameraPlugin {
            is_editor: self.is_editor,
        });
        app.add_plugin(LevelProgressPlugin);
        app.add_plugin(PlayerPlugin);
        app.add_plugin(WallPlugin);
        app.add_plugin(RobotPartPlugin);

        app.add_plugin(PlayerControlPlugin);
        app.add_plugin(PartsManipulationPlugin);

        app.add_system(enable_disable_physics);
        if self.is_editor {
            app.add_plugin(YoleckSyncWithEditorState {
                when_editor: AppState::Editor,
                when_game: AppState::Game,
            });
        } else {
            app.add_plugin(MenuPlugin);
            app.add_state(AppState::Menu(MenuState::Main));
            app.add_system_set(
                SystemSet::on_enter(AppState::LoadLevel).with_system(handle_level_loading),
            );
            if let Some(start_at_level) = &self.start_at_level {
                let start_at_level = format!("{}.yol", start_at_level);
                app.add_startup_system(
                    move |mut level_progress: ResMut<LevelProgress>,
                          mut state: ResMut<State<AppState>>| {
                        level_progress.current_level = Some(start_at_level.clone());
                        state.overwrite_set(AppState::LoadLevel).unwrap();
                    },
                );
            }
        }
    }
}

fn enable_disable_physics(
    state: Res<State<AppState>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    rapier_configuration.physics_pipeline_active = *state.current() == AppState::Game;
}

fn handle_level_loading(
    level_entities_query: Query<Entity, With<YoleckManaged>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_progress: Res<LevelProgress>,
    mut yoleck_loading_command: ResMut<YoleckLoadingCommand>,
    mut state: ResMut<State<AppState>>,
) {
    for entity in level_entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    let current_level = level_progress
        .current_level
        .as_ref()
        .expect("Entered LoadLevel state when current_level is None");
    *yoleck_loading_command =
        YoleckLoadingCommand::FromAsset(asset_server.load(&format!("levels/{}", current_level)));
    state.overwrite_set(AppState::Game).unwrap();
}
