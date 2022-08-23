use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use float_ord::FloatOrd;
use leafwing_input_manager::prelude::ActionState;

use crate::global_types::{AppState, Carrier, HalfHeight, InputBinding, Pickable};
use crate::physics_utils::standing_on;
use crate::utils::some_or;

pub struct PartsManipulationPlugin;

impl Plugin for PartsManipulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set({ SystemSet::on_update(AppState::Game).with_system(control_pickup) });
    }
}

fn control_pickup(
    mut player_query: Query<(&ActionState<InputBinding>, Entity, &mut Carrier)>,
    mut pickable_query: Query<&mut Pickable>,
    mut transform_query: Query<(&mut Transform, &HalfHeight)>,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
) {
    let mut swap_places = |top, bottom| {
        transform_query.get_many_mut([top, bottom]).map(move |query_result| {
            let [
                (mut top_transform, HalfHeight(top_hh)),
                (mut bot_transform, HalfHeight(bot_hh)),
            ] = query_result;
            bot_transform.translation.y += 0.01 + 2.0 * top_hh;
            top_transform.translation.y -= 0.01 + 2.0 * bot_hh;
            top_hh + bot_hh
        })
    };

    for (action_state, player_entity, mut carrier) in player_query.iter_mut() {
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
        if let Some(pickable_entity) = carrier.carrying {
            let mut pickable = pickable_query
                .get_mut(pickable_entity)
                .expect("Player should only be able to carry pickable entities");
            commands.entity(pickable_entity).remove::<ImpulseJoint>();
            carrier.carrying = None;
            pickable.carried_by = None;
        } else if let Some((_offset_this, _offset_that, standing_on_entity)) = standing_on {
            let mut pickable = some_or!(pickable_query.get_mut(standing_on_entity).ok(); continue);
            let pickable_entity = standing_on_entity;
            if let Ok(combined_half_height) = swap_places(player_entity, pickable_entity) {
                let joint = FixedJointBuilder::new()
                    .local_anchor1(Vec2::new(0.0, 0.01 + combined_half_height));
                commands
                    .entity(pickable_entity)
                    .insert(ImpulseJoint::new(player_entity, joint));

                carrier.carrying = Some(pickable_entity);
                pickable.carried_by = Some(player_entity);
            }
        }
    }
}
