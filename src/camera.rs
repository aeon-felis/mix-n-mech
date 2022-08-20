use bevy::prelude::*;
use bevy_egui::EguiSettings;
use bevy_yoleck::YoleckEditorState;

pub struct CameraPlugin {
    pub is_editor: bool,
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
        app.add_system_set(
            SystemSet::on_update(YoleckEditorState::GameActive)
                .with_system(update_camera_transform),
        );
        app.add_system_set(
            SystemSet::on_enter(YoleckEditorState::EditorActive).with_system(
                |mut egui_settings: ResMut<EguiSettings>| {
                    egui_settings.scale_factor = 1.0;
                },
            ),
        );
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation.z = 100.0;
    camera.transform.scale = Vec3::new(0.016, 0.016, 1.0);
    commands.spawn_bundle(camera);
}

fn update_camera_transform() {
    // TODO: implement
}
