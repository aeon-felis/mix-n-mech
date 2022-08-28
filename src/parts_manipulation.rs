use bevy::ecs::query::QueryEntityError;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use float_ord::FloatOrd;
use leafwing_input_manager::prelude::ActionState;

use crate::global_types::{AppState, Carrier, HalfHeight, InputBinding, IsMountBase, Pickable};
use crate::physics_utils::standing_on;
use crate::utils::some_or;

pub struct PartsManipulationPlugin;

impl Plugin for PartsManipulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(apply_carrying)
                .with_system(control_pickup)
                .with_system(detect_mounting)
        });
    }
}

#[derive(Component)]
struct ChangeCarrying {
    carrier_entity: Entity,
    old_carrier_entity: Option<Entity>,
}

#[derive(SystemParam)]
struct SwapPlaces<'w, 's> {
    query: Query<'w, 's, (&'static mut Transform, &'static HalfHeight)>,
}

impl SwapPlaces<'_, '_> {
    fn swap_places(&mut self, top: Entity, bottom: Entity) -> Result<f32, QueryEntityError> {
        self.query.get_many_mut([top, bottom]).map(move |query_result| {
            let [
                (mut top_transform, HalfHeight(top_hh)),
                (mut bot_transform, HalfHeight(bot_hh)),
            ] = query_result;
            bot_transform.translation.y += 0.01 + 2.0 * top_hh;
            top_transform.translation.y -= 0.01 + 2.0 * bot_hh;
            top_hh + bot_hh
        })
    }
}

fn control_pickup(
    mut player_query: Query<(&ActionState<InputBinding>, Entity), With<Carrier>>,
    mut pickable_query: Query<(&mut Pickable, &mut Transform, &mut Velocity)>,
    mut carrier_query: Query<&mut Carrier>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
) {
    for (action_state, player_entity) in player_query.iter_mut() {
        if !action_state.just_pressed(InputBinding::Pickup) {
            continue;
        }
        let standing_on = standing_on(&rapier_context, player_entity, |ed| {
            let (offset_this, offset_that) = ed
                .manifold
                .points
                .iter()
                .map(|point| {
                    let [this, that] = ed.maybe_swap([point.local_p1, point.local_p2]);
                    (this.y, that.y)
                })
                .min_by_key(|(a, b)| (FloatOrd(*a), FloatOrd(-*b)))
                .unwrap();
            (offset_this, offset_that, ed.other)
        });
        let mut carrier = carrier_query.get_mut(player_entity).unwrap();
        if let Some(pickable_entity) = carrier.carrying {
            let throw_direction = action_state
                .axis_pair(InputBinding::Move)
                .and_then(|input| {
                    let x = input.x();
                    if x.abs() < 0.5 {
                        None
                    } else {
                        Some(x.signum())
                    }
                });
            let throw_direction = some_or!(throw_direction; continue);
            let (mut pickable, mut pickable_transform, mut pickable_velocity) = pickable_query
                .get_mut(pickable_entity)
                .expect("Player should only be able to carry pickable entities");
            pickable_transform.translation += Vec3::new(0.75 * throw_direction, -0.2, 0.0);
            pickable_velocity.linvel += Vec2::new(0.0, -3.0);
            commands.entity(pickable_entity).remove::<ImpulseJoint>();
            carrier.carrying = None;
            pickable.carried_by = None;
        } else if let Some((_offset_this, _offset_that, standing_on_entity)) = standing_on {
            let (pickable, _, _) =
                some_or!(pickable_query.get_mut(standing_on_entity).ok(); continue);
            let pickable_entity = standing_on_entity;

            commands.entity(pickable_entity).remove::<ImpulseJoint>();

            commands.entity(pickable_entity).insert(ChangeCarrying {
                carrier_entity: player_entity,
                old_carrier_entity: pickable.carried_by,
            });
        }
    }
}

fn detect_mounting(
    mut reader: EventReader<CollisionEvent>,
    mut carrier_query: Query<(&mut Carrier, &HalfHeight), With<IsMountBase>>,
    mut pickable_query: Query<(&mut Pickable, &HalfHeight)>,
    mut transform_query: Query<&mut Transform>,
    global_transform_query: Query<&GlobalTransform>,
    mut commands: Commands,
) {
    for event in reader.iter() {
        if let &CollisionEvent::Started(e1, e2, _) = event {
            let [carrier_entity, pickable_entity] =
                if let Ok(transforms) = global_transform_query.get_many([e1, e2]) {
                    let [t1, t2] = transforms.map(|t| t.translation());
                    if 0.5 < (t1.x - t2.x).abs() {
                        continue;
                    }
                    if t1.y < t2.y {
                        [e1, e2]
                    } else {
                        [e2, e1]
                    }
                } else {
                    continue;
                };
            let (mut carrier, HalfHeight(carrier_hh)) =
                some_or!(carrier_query.get_mut(carrier_entity).ok(); continue);
            if carrier.carrying.is_some() {
                continue;
            }
            let (mut pickable, HalfHeight(pickable_hh)) =
                some_or!(pickable_query.get_mut(pickable_entity).ok(); continue);
            if pickable.carried_by.is_some() {
                continue;
            }

            let [mut pickable_transform, carrier_transform] = transform_query
                .get_many_mut([pickable_entity, carrier_entity])
                .unwrap();
            pickable_transform.translation.x = carrier_transform.translation.x;

            carrier.carrying = Some(pickable_entity);
            pickable.carried_by = Some(carrier_entity);
            let joint = FixedJointBuilder::new()
                .local_anchor1(Vec2::new(0.0, 0.01 + carrier_hh + pickable_hh));
            commands
                .entity(pickable_entity)
                .insert(ImpulseJoint::new(carrier_entity, joint));
        }
    }
}

fn apply_carrying(
    mut pickable_query: Query<(Entity, &ChangeCarrying, &mut Pickable)>,
    mut carrier_query: Query<&mut Carrier>,
    mut swap_places: SwapPlaces,
    mut commands: Commands,
) {
    for (
        pickable_entity,
        &ChangeCarrying {
            carrier_entity,
            old_carrier_entity,
        },
        mut pickable,
    ) in pickable_query.iter_mut()
    {
        let mut carrier = some_or!(carrier_query.get_mut(carrier_entity).ok(); continue);
        if let Ok(combined_hh) = swap_places.swap_places(carrier_entity, pickable_entity) {
            let joint = FixedJointBuilder::new().local_anchor1(Vec2::new(0.0, 0.01 + combined_hh));
            commands
                .entity(pickable_entity)
                .remove::<ChangeCarrying>()
                .insert(ImpulseJoint::new(carrier_entity, joint));
            pickable.carried_by = Some(carrier_entity);
            carrier.carrying = Some(pickable_entity);
            if let Some(mut old_carrier) =
                old_carrier_entity.and_then(|e| carrier_query.get_mut(e).ok())
            {
                old_carrier.carrying = None;
            }
        }
    }
}
