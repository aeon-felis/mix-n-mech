mod camera;
mod wall;
mod yoleck_utils;

use bevy::prelude::*;

use self::camera::CameraPlugin;
use self::wall::WallPlugin;

pub struct GamePlugin {
    pub is_editor: bool,
    pub start_at_level: Option<String>,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(CameraPlugin {
            is_editor: self.is_editor,
        });
        app.add_plugin(WallPlugin);
    }
}
