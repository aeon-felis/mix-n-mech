use bevy::prelude::*;
use bevy::utils::HashSet;

use crate::global_types::{Activatable, AppState, Carrier, HDirection, IsPowerSource};
use crate::utils::some_or;

pub struct PartActivationPlugin;

impl Plugin for PartActivationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(set_sprite_properties);
        app.add_system_set({
            SystemSet::on_update(AppState::Game).with_system(set_activation_state)
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

fn set_activation_state(
    mount_base_query: Query<(Entity, &Carrier), With<IsPowerSource>>,
    carrier_query: Query<&Carrier>,
    mut activatable_query: Query<(Entity, &mut Activatable)>,
) {
    let mut parts_to_activate = HashSet::<Entity>::new();

    for (mount_base_entity, mount_base_carrier) in mount_base_query.iter() {
        if let Some(mut entity) = mount_base_carrier.carrying {
            parts_to_activate.insert(mount_base_entity);
            parts_to_activate.insert(entity);
            while let Ok(carrier) = carrier_query.get(entity) {
                entity = some_or!(carrier.carrying; break);
                let is_new_in_set = parts_to_activate.insert(entity);
                if !is_new_in_set {
                    break;
                }
            }
        }
    }

    for (entity, mut activatable) in activatable_query.iter_mut() {
        activatable.active = parts_to_activate.contains(&entity);
    }
}
