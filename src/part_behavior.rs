use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier2d::prelude::*;

use crate::global_types::{Activatable, AppState, Carrier, HDirection};
use crate::laser::TriggerLaserShot;
use crate::utils::some_or;

pub struct PartBehaviorPlugin;

impl Plugin for PartBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(impl_hover)
                .with_system(impl_laser)
                .with_system(impl_rotator)
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
        &Children,
    )>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, activatable, behavior, transform, mut velocity, children) in hover_query.iter_mut()
    {
        if !activatable.active {
            continue;
        }
        let ignore: HashSet<Entity> = std::iter::once(entity)
            .chain(children.iter().copied())
            .collect();
        if let Some((_other_entity, toi)) = rapier_context.cast_shape(
            transform.translation.truncate(),
            0.0,
            Vec2::NEG_Y,
            &Collider::cuboid(0.25, 0.1),
            behavior.range,
            QueryFilter::default().predicate(&|other_entity| !ignore.contains(&other_entity)),
        ) {
            let desired_up_speed = 2.0 * (behavior.range - toi.toi);
            if velocity.linvel.y < desired_up_speed {
                velocity.linvel.y = 0.5 * (velocity.linvel.y + desired_up_speed);
            }
        }
    }
}

#[derive(Component)]
pub struct LaserBehavior {
    pub next_shot_timer: Timer,
    pub speed: f32,
    pub range: f32,
}

fn impl_laser(
    mut laser_query: Query<(
        Entity,
        &Activatable,
        &mut LaserBehavior,
        &Transform,
        &HDirection,
    )>,
    time: Res<Time>,
    mut trigger_laser_shot_writer: EventWriter<TriggerLaserShot>,
) {
    for (entity, activatable, mut behavior, transform, hdirection) in laser_query.iter_mut() {
        if !activatable.active {
            behavior.next_shot_timer.reset();
            continue;
        }
        behavior.next_shot_timer.tick(time.delta());
        if !behavior.next_shot_timer.just_finished() {
            continue;
        }
        trigger_laser_shot_writer.send(TriggerLaserShot {
            ignore_entity: entity,
            origin: transform.translation.truncate() + 0.5 * hdirection.as_vec(),
            velocity: behavior.speed * hdirection.as_vec(),
            range: behavior.range,
        })
    }
}

#[derive(Component)]
pub struct RotatorBehavior {
    pub next_turn_timer: Timer,
}

fn impl_rotator(
    mut rotator_query: Query<(Entity, &Activatable, &mut RotatorBehavior)>,
    mut rotating_part_query: Query<(&mut HDirection, Option<&Carrier>)>,
    time: Res<Time>,
) {
    for (mut entity, activatable, mut behavior) in rotator_query.iter_mut() {
        if !activatable.active {
            behavior.next_turn_timer.reset();
            continue;
        }
        behavior.next_turn_timer.tick(time.delta());
        if !behavior.next_turn_timer.just_finished() {
            continue;
        }
        loop {
            let (mut hdirection, carrier) =
                some_or!(rotating_part_query.get_mut(entity).ok(); break);
            *hdirection = hdirection.switch();
            let carrier = some_or!(carrier; break);
            entity = some_or!(carrier.carrying; break);
        }
    }
}
