use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
use bevy_yoleck::{YoleckExtForApp, YoleckPopulate, YoleckTypeHandler};
use serde::{Deserialize, Serialize};

use crate::global_types::{AppState, IsDoorKey, IsPlayer, OpenableDoor};
use crate::loading::GameAssets;
use crate::utils::{entities_ordered_by_type, some_or};

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_yoleck_handler({
            YoleckTypeHandler::<Door>::new("Door")
                .populate_with(populate)
                .with(crate::yoleck_utils::position_adapter(
                    |door: &mut Door| (&mut door.position, IVec2::ONE),
                    0.0,
                ))
        });
        app.add_system_set({
            SystemSet::on_update(AppState::Game).with_system(handle_opening_when_keys_are_taken)
        });
        app.add_system(handle_player_enters);
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Door {
    #[serde(default)]
    position: Vec2,
}

fn populate(mut populate: YoleckPopulate<Door>, game_assets: Res<GameAssets>) {
    populate.populate(|_ctx, _data, mut cmd| {
        cmd.insert_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            texture_atlas: game_assets.door.clone(),
            ..Default::default()
        });
        cmd.insert(RigidBody::Fixed);
        cmd.insert(Collider::cuboid(0.5, 0.5));
        cmd.insert(Sensor);
        cmd.insert(ActiveEvents::COLLISION_EVENTS);
        cmd.insert(OpenableDoor { open: false });
    });
}

fn handle_opening_when_keys_are_taken(
    keys_query: Query<(), With<IsDoorKey>>,
    mut door_query: Query<(&mut OpenableDoor, &mut TextureAtlasSprite)>,
) {
    let should_be_open = keys_query.is_empty();
    let sprite_index = if should_be_open { 1 } else { 0 };
    for (mut door, mut sprite) in door_query.iter_mut() {
        door.open = should_be_open;
        sprite.index = sprite_index;
    }
}

fn handle_player_enters(
    mut reader: EventReader<CollisionEvent>,
    player_query: Query<(), With<IsPlayer>>,
    door_query: Query<&OpenableDoor>,
    mut state: ResMut<State<AppState>>,
) {
    for event in reader.iter() {
        if let &CollisionEvent::Started(e1, e2, CollisionEventFlags::SENSOR) = event {
            let [_player_entity, door_entity] = some_or!(
                entities_ordered_by_type!([e1, e2], player_query, door_query);
                continue);
            let door = door_query.get(door_entity).unwrap();
            if door.open {
                state.set(AppState::LevelCompleted).unwrap();
                return;
            }
        }
    }
}
