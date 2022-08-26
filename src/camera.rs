use bevy::prelude::*;
use bevy::text::Text2dSize;
use bevy_egui::EguiSettings;
use bevy_yoleck::{YoleckEditorState, YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::global_types::CameraInclude;
use crate::utils::some_or;

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
        app.add_yoleck_handler({
            YoleckTypeHandler::<CameraMarker>::new("CameraMarker")
                .populate_with(populate_camera_marker)
                .with(crate::yoleck_utils::position_adapter(
                    |camera_marker: &mut CameraMarker| (&mut camera_marker.position, IVec2::ONE),
                    0.0,
                ))
        });
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation.z = 100.0;
    camera.transform.scale = Vec3::new(0.016, 0.016, 1.0);
    commands.spawn_bundle(camera);
}

#[allow(clippy::type_complexity)]
fn update_camera_transform(
    mut cameras_query: Query<(&mut Transform, &OrthographicProjection), With<Camera>>,
    camera_included_objects_query: Query<
        (&GlobalTransform, AnyOf<(&Sprite, &Text2dSize)>),
        With<CameraInclude>,
    >,
) {
    let mut minmax: Option<[f32; 4]> = None;
    for (global_transform, (sprite, text_2d_size)) in camera_included_objects_query.iter() {
        let (vec_to_min, vec_to_max) = if let Some(sprite) = sprite {
            let vec = 0.5 * sprite.custom_size.unwrap().extend(0.0);
            (-vec, vec)
        } else if let Some(text_2d_size) = text_2d_size {
            (Vec3::ZERO, text_2d_size.size.extend(0.0))
        } else {
            panic!("No option for calculating the size");
        };
        let min_corner = global_transform.mul_vec3(vec_to_min);
        let max_corner = global_transform.mul_vec3(vec_to_max);
        minmax = if let Some([l, b, r, t]) = minmax {
            Some([
                l.min(min_corner.x),
                b.min(min_corner.y),
                r.max(max_corner.x),
                t.max(max_corner.y),
            ])
        } else {
            Some([min_corner.x, min_corner.y, max_corner.x, max_corner.y])
        };
    }
    let minmax = some_or!(minmax; return);
    let world_width = minmax[2] - minmax[0];
    let world_height = minmax[3] - minmax[1];
    for (mut transform, projection) in cameras_query.iter_mut() {
        let projection_width = projection.right - projection.left;
        let projection_height = projection.top - projection.bottom;
        let width_ratio = world_width / projection_width;
        let height_ratio = world_height / (projection_height - 50.0);
        let chosen_ratio = width_ratio.max(height_ratio) * 1.1;
        transform.scale = Vec3::new(chosen_ratio, chosen_ratio, 1.0);
        transform.translation.x = 0.5 * (minmax[0] + minmax[2]);
        transform.translation.y = 0.5 * (minmax[1] + minmax[3]) + 50.0 * chosen_ratio;
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraMarker {
    #[serde(default)]
    position: Vec2,
}

fn populate_camera_marker(mut populate: YoleckPopulate<CameraMarker>) {
    populate.populate(|ctx, _data, mut cmd| {
        cmd.insert(CameraInclude);
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::PURPLE.clone().set_a(0.5).to_owned(),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            ..Default::default()
        });
        cmd.insert(Visibility {
            is_visible: ctx.is_in_editor(),
        });
    });
}
