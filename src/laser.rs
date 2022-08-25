use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::global_types::AppState;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TriggerLaserShot>();
        app.add_system(shoot_laser);
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(dispose_laser));
    }
}

#[derive(Debug)]
pub struct TriggerLaserShot {
    pub origin: Vec2,
    pub velocity: Vec2,
    pub range: f32,
}

fn shoot_laser(mut reader: EventReader<TriggerLaserShot>, mut commands: Commands) {
    for event in reader.iter() {
        let mut cmd = commands.spawn();
        cmd.insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW_GREEN,
                custom_size: Some(Vec2::new(0.2, 0.2)),
                ..Default::default()
            },
            transform: Transform::from_translation(event.origin.extend(1.0)),
            ..Default::default()
        });
        cmd.insert(RigidBody::KinematicVelocityBased);
        cmd.insert(Velocity::linear(event.velocity));

        cmd.insert(Laser {
            origin: event.origin,
            range: event.range,
        });
    }
}

#[derive(Component)]
pub struct Laser {
    origin: Vec2,
    range: f32,
}

fn dispose_laser(query: Query<(Entity, &Laser, &Transform)>, mut commands: Commands) {
    for (entity, laser, transform) in query.iter() {
        let distance = laser.origin.distance(transform.translation.truncate());
        if laser.range <= distance {
            commands.entity(entity).despawn_recursive();
        }
    }
}
