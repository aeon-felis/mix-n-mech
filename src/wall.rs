use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::{egui, YoleckEdit, YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::global_types::CameraInclude;
use crate::laser::Breakable;
use crate::yoleck_utils::GRANULARITY;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandler::<Wall>::new("Wall")
                .populate_with(populate)
                .with(crate::yoleck_utils::position_adapter(
                    |wall: &mut Wall| (&mut wall.position, wall.size),
                    0.0,
                ))
                .edit_with(edit)
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Wall {
    #[serde(default)]
    position: Vec2,
    #[serde(default = "default_size")]
    size: IVec2,
    #[serde(default)]
    breakable: bool,
}

fn default_size() -> IVec2 {
    IVec2::new(1, 1)
}

fn populate(mut populate: YoleckPopulate<Wall>) {
    populate.populate(|_ctx, data, mut cmd| {
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: if data.breakable {
                    Color::rgb(0.6, 0.6, 0.6)
                } else {
                    Color::DARK_GRAY
                },
                custom_size: Some(data.size.as_vec2()),
                ..Default::default()
            },
            ..Default::default()
        });
        cmd.insert(CameraInclude);
        cmd.insert(RigidBody::Fixed);
        cmd.insert(Collider::cuboid(
            data.size.x as f32 * 0.5,
            data.size.y as f32 * 0.5,
        ));

        if data.breakable {
            cmd.insert(Breakable::default());
        }
    });
}

fn edit(mut edit: YoleckEdit<Wall>, mut commands: Commands) {
    edit.edit(|ctx, data, ui| {
        ui.checkbox(&mut data.breakable, "Breakable?");

        for move_anchor in [(false, false), (false, true), (true, false), (true, true)] {
            let mut resize_knob = ctx.knob(&mut commands, ("resize", move_anchor));
            let anchor_offset = IVec2::new(move_anchor.0 as i32, move_anchor.1 as i32).as_vec2();
            let anchor_position =
                data.position + (anchor_offset * data.size.as_vec2()) * GRANULARITY;
            let knob_position =
                data.position + ((Vec2::ONE - anchor_offset) * data.size.as_vec2()) * GRANULARITY;
            resize_knob.cmd.insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::ORANGE_RED,
                    custom_size: Some(Vec2::new(0.3, 0.3)),
                    ..Default::default()
                },
                transform: Transform::from_translation(knob_position.extend(1.0)),
                global_transform: Transform::from_translation(knob_position.extend(1.0)).into(),
                ..Default::default()
            });
            if let Some(new_knob_pos) = resize_knob.get_passed_data::<Vec2>() {
                let new_size = *new_knob_pos - anchor_position;

                for (is_anchor, new_dim, dim, coord) in [
                    (
                        move_anchor.0,
                        new_size.x,
                        &mut data.size.x,
                        &mut data.position.x,
                    ),
                    (
                        move_anchor.1,
                        new_size.y,
                        &mut data.size.y,
                        &mut data.position.y,
                    ),
                ] {
                    let mut new_dim = new_dim.round();
                    if is_anchor {
                        new_dim = -new_dim;
                    }
                    let new_dim = f32::max(new_dim, GRANULARITY) as i32;
                    if is_anchor && new_dim != *dim {
                        *coord += (*dim - new_dim) as f32;
                    }
                    *dim = new_dim;
                }
            }
        }

        ui.horizontal(|ui| {
            for (caption, value) in [("Width:", &mut data.size.x), ("Height:", &mut data.size.y)] {
                ui.add(egui::DragValue::new(value).prefix(caption).speed(0.05));
            }
        });
    });
}
