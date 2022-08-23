use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::rapier::prelude::ContactManifold;
use float_ord::FloatOrd;

pub struct ExtractData<'a> {
    pub normal: Vec2,
    pub other: Entity,
    should_swap: bool,
    pub manifold: &'a ContactManifold,
}

impl ExtractData<'_> {
    pub fn maybe_swap<T>(&self, items: [T; 2]) -> [T; 2] {
        if self.should_swap {
            let [a, b] = items;
            [b, a]
        } else {
            items
        }
    }
}
pub fn standing_on<T>(
    rapier_context: &RapierContext,
    entity: Entity,
    mut extract_dlg: impl FnMut(&ExtractData) -> T,
) -> Option<T> {
    rapier_context
        .contacts_with(entity)
        .filter(|contact| contact.raw.has_any_active_contact)
        .filter_map(|contact| {
            contact
                .manifolds()
                .filter_map(|contact_manifold| {
                    let extract_data = if contact.collider1() == entity {
                        ExtractData {
                            normal: -contact_manifold.normal(),
                            other: contact.collider2(),
                            should_swap: false,
                            manifold: contact_manifold.raw,
                        }
                    } else if contact.collider2() == entity {
                        ExtractData {
                            normal: contact_manifold.normal(),
                            other: contact.collider1(),
                            should_swap: true,
                            manifold: contact_manifold.raw,
                        }
                    } else {
                        return None;
                    };
                    Some((extract_dlg(&extract_data), extract_data.normal))
                })
                .max_by_key(|(_, normal)| FloatOrd(normal.dot(Vec2::Y)))
        })
        .max_by_key(|(_, normal)| FloatOrd(normal.dot(Vec2::Y)))
        .map(|(result, _)| result)
}
