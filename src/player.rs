use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_yoleck::{YoleckEdit, YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use leafwing_input_manager::prelude::{DualAxis, InputMap, VirtualDPad};
use leafwing_input_manager::InputManagerBundle;
use serde::{Deserialize, Serialize};

use crate::global_types::{Carrier, HalfHeight, InputBinding, IsPlayer};
use crate::loading::GameAssets;
use crate::player_control::PlayerControl;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandler::<Player>::new("Player")
                .populate_with(populate)
                .with(crate::yoleck_utils::position_adapter(
                    |player: &mut Player| (&mut player.position, IVec2::ONE),
                    0.0,
                ))
                .edit_with(edit)
                .populate_with(add_player_input)
        });
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    #[serde(default)]
    position: Vec2,
}

fn populate(mut populate: YoleckPopulate<Player>, game_assets: Res<GameAssets>) {
    populate.populate(|_ctx, _data, mut cmd| {
        cmd.insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            texture_atlas: game_assets.player.clone(),
            ..Default::default()
        });
        cmd.insert(IsPlayer);
        cmd.insert(PlayerControl::default());
        cmd.insert(Carrier::default());
        cmd.insert(HalfHeight(0.5));

        cmd.insert(RigidBody::Dynamic);
        cmd.insert(Collider::cuboid(0.25, 0.5));
        cmd.insert(ColliderMassProperties::Density(10.0));
        cmd.insert(Velocity::default());
        cmd.insert(LockedAxes::ROTATION_LOCKED);
    });
}

fn edit(mut edit: YoleckEdit<Player>, mut _commands: Commands) {
    edit.edit(|_ctx, _data, _ui| {});
}

fn add_player_input(mut populate: YoleckPopulate<Player>) {
    populate.populate(|ctx, _data, mut cmd| {
        if ctx.is_in_editor() {
            return;
        }
        cmd.insert_bundle(InputManagerBundle {
            action_state: Default::default(),
            input_map: InputMap::default()
                .insert(VirtualDPad::arrow_keys(), InputBinding::Move)
                .insert(VirtualDPad::dpad(), InputBinding::Move)
                .insert(DualAxis::left_stick(), InputBinding::Move)
                .insert(KeyCode::Space, InputBinding::Pickup)
                .insert(GamepadButtonType::South, InputBinding::Pickup)
                .build(),
        });
    });
}
