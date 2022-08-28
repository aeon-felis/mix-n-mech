use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use bevy_yoleck::{YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::global_types::{IsDoorKey, IsPlayer};
use crate::loading::GameAssets;
use crate::utils::{entities_ordered_by_type, some_or};

pub struct DoorKeyPlugin;

impl Plugin for DoorKeyPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandler::<DoorKey>::new("DoorKey")
                .populate_with(populate)
                .with(crate::yoleck_utils::position_adapter(
                    |door_key: &mut DoorKey| (&mut door_key.position, IVec2::ONE),
                    0.0,
                ))
        });
        app.add_system(handle_taken_by_player);
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct DoorKey {
    #[serde(default)]
    position: Vec2,
}

fn populate(mut populate: YoleckPopulate<DoorKey>, game_assets: Res<GameAssets>) {
    populate.populate(|_ctx, _data, mut cmd| {
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.5, 0.5)),
                ..Default::default()
            },
            texture: game_assets.door_key.clone(),
            ..Default::default()
        });
        cmd.insert(RigidBody::Fixed);
        cmd.insert(Collider::cuboid(0.101562, 0.25));
        cmd.insert(Sensor);
        cmd.insert(ActiveEvents::COLLISION_EVENTS);
        cmd.insert(IsDoorKey);
    });
}

fn handle_taken_by_player(
    mut reader: EventReader<CollisionEvent>,
    player_query: Query<(), With<IsPlayer>>,
    door_key_query: Query<(), With<IsDoorKey>>,
    mut commands: Commands,
) {
    for event in reader.iter() {
        if let &CollisionEvent::Started(e1, e2, CollisionEventFlags::SENSOR) = event {
            let [_player_entity, door_key_entity] = some_or!(
                entities_ordered_by_type!([e1, e2], player_query, door_key_query);
                continue);
            commands.entity(door_key_entity).despawn_recursive();
        }
    }
}
