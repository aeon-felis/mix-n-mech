use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy_rapier2d::prelude::*;

use crate::global_types::{Activatable, AppState, HDirection};
use crate::laser::TriggerLaserShot;

pub struct PartBehaviorPlugin;

impl Plugin for PartBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(set_sprite_properties);
        app.add_system_set({
            SystemSet::on_update(AppState::Game)
                .with_system(impl_hover)
                .with_system(impl_laser)
        });
    }
}

fn set_sprite_properties(
    mut query: Query<(&mut TextureAtlasSprite, &HDirection, Option<&Activatable>)>,
) {
    for (mut sprite, hdirection, activatable) in query.iter_mut() {
        sprite.flip_x = *hdirection == HDirection::Left;
        if let Some(activatable) = activatable {
            sprite.index = if activatable.active { 1 } else { 0 };
        }
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
        if behavior.next_shot_timer.just_finished() {
            trigger_laser_shot_writer.send(TriggerLaserShot {
                ignore_entity: entity,
                origin: transform.translation.truncate() + 0.5 * hdirection.as_vec(),
                velocity: behavior.speed * hdirection.as_vec(),
                range: behavior.range,
            })
        }
    }
}
