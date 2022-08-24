use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::{egui, YoleckEdit, YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::global_types::{Activatable, Carrier, HalfHeight, IsMountBase, Pickable};
use crate::loading::GameAssets;
use crate::part_behavior::HoverBehavior;

pub struct RobotPartPlugin;

impl Plugin for RobotPartPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandler::<RobotPart>::new("RobotPart")
                .populate_with(populate)
                .with(crate::yoleck_utils::position_adapter(
                    |robot_part: &mut RobotPart| (&mut robot_part.position, IVec2::ONE),
                    0.0,
                ))
                .edit_with(edit)
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotPart {
    #[serde(default)]
    position: Vec2,
    #[serde(default = "default_type")]
    part_type: RobotPartType,
}

fn default_type() -> RobotPartType {
    RobotPartType::Platform
}

fn populate(mut populate: YoleckPopulate<RobotPart>, _game_assets: Res<GameAssets>) {
    populate.populate(|ctx, data, mut cmd| {
        let part_height = data.part_type.height();
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: data.part_type.color(),
                custom_size: Some(Vec2::new(1.0, part_height)),
                ..Default::default()
            },
            ..Default::default()
        });
        cmd.insert(HalfHeight(0.5 * part_height));

        cmd.insert(RigidBody::Dynamic);
        cmd.insert(Collider::cuboid(0.5, 0.5 * part_height));
        cmd.insert(ColliderMassProperties::Density(100.0));
        cmd.insert(Velocity::default());
        cmd.insert(LockedAxes::ROTATION_LOCKED);

        if !ctx.is_in_editor() {
            data.part_type.fill_components(&mut cmd);
        }
    });
}

fn edit(mut edit: YoleckEdit<RobotPart>, mut _commands: Commands) {
    edit.edit(|_ctx, data, ui| {
        egui::ComboBox::from_id_source("part_type")
            .selected_text(format!("{:?}", data.part_type))
            .show_ui(ui, |ui| {
                for possible_type in RobotPartType::list() {
                    ui.selectable_value(
                        &mut data.part_type,
                        *possible_type,
                        format!("{:?}", possible_type),
                    );
                }
            });
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RobotPartType {
    Platform,
    Hover,
    Laser,
}

impl RobotPartType {
    fn list() -> &'static [RobotPartType] {
        &[Self::Platform, Self::Hover, Self::Laser]
    }

    fn height(&self) -> f32 {
        match self {
            RobotPartType::Platform => 0.2,
            RobotPartType::Hover => 0.2,
            RobotPartType::Laser => 0.6,
        }
    }

    fn color(&self) -> Color {
        match self {
            RobotPartType::Platform => Color::BEIGE,
            RobotPartType::Hover => Color::BLUE,
            RobotPartType::Laser => Color::YELLOW,
        }
    }

    fn fill_components(&self, cmd: &mut EntityCommands) {
        match self {
            RobotPartType::Platform => {
                cmd.insert(Pickable::default());
            }
            RobotPartType::Hover => {
                cmd.insert(IsMountBase);
                cmd.insert(Carrier::default());
                cmd.insert(ActiveEvents::COLLISION_EVENTS);
                cmd.insert(Activatable { active: false });
                cmd.insert(HoverBehavior { range: 0.5 });
            }
            RobotPartType::Laser => {
                cmd.insert(Pickable::default());
                cmd.insert(Activatable { active: false });
            }
        }
    }
}
