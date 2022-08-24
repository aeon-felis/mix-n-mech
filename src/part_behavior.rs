use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::{Activatable, AppState};

pub struct PartBehaviorPlugin;

impl Plugin for PartBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(impl_hover)
                .with_system(impl_laser)
        });
    }
}

#[derive(Component)]
pub struct HoverBehavior {
    pub range: f32,
}

fn impl_hover(
    mut hover_query: Query<(
        Entity,
        &Activatable,
        &HoverBehavior,
        &Transform,
        &mut Velocity,
    )>,
    rapier_context: Res<RapierContext>,
) {
    for (hover_entity, activatable, hover_behavior, transform, mut velocity) in
        hover_query.iter_mut()
    {
        if !activatable.active {
            continue;
        }
        if let Some((_other_entity, toi)) = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::NEG_Y,
            &Collider::cuboid(0.25, 0.1),
            hover_behavior.range,
            QueryFilter::default().predicate(&|other_entity| other_entity != hover_entity),
        ) {
            let desired_up_speed = 2.0 * (hover_behavior.range - toi.toi);
            if velocity.linvel.y < desired_up_speed {
                velocity.linvel.y = 0.5 * (velocity.linvel.y + desired_up_speed);
            }
        }
    }
}

fn impl_laser() {
    // TODO: implement
}
