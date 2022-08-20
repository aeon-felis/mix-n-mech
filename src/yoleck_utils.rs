use bevy::prelude::*;
use bevy_egui::egui;
use bevy_yoleck::{YoleckEdit, YoleckPopulate, YoleckTypeHandler};

pub const GRANULARITY: f32 = 1.0;

pub fn round_to_tick(number: f32, tick: f32) -> f32 {
    (number / tick).round() * tick
}

pub fn round_vec2_to_tick(vec: Vec2, tick: f32) -> Vec2 {
    Vec2::new(round_to_tick(vec.x, tick), round_to_tick(vec.y, tick))
}

pub fn position_adapter<T: 'static>(
    projection: impl 'static + Clone + Send + Sync + Fn(&mut T) -> (&mut Vec2, IVec2),
    z: f32,
) -> impl FnOnce(YoleckTypeHandler<T>) -> YoleckTypeHandler<T> {
    move |handler| {
        handler
            .populate_with({
                let projection = projection.clone();
                move |mut populate: YoleckPopulate<T>| {
                    populate.populate(|_ctx, data, mut cmd| {
                        let (position, size) = projection(data);
                        let center_position = *position + (0.5 * GRANULARITY) * size.as_vec2();
                        cmd.insert(Transform::from_translation(center_position.extend(z)));
                    });
                }
            })
            .edit_with(move |mut edit: YoleckEdit<T>| {
                edit.edit(|ctx, data, ui| {
                    let (position, size) = projection(data);
                    if let Some(pos) = ctx.get_passed_data::<Vec2>() {
                        let pos = *pos;
                        let pos = pos - 0.5 * GRANULARITY * size.as_vec2();
                        *position = pos;
                    }
                    ui.horizontal(|ui| {
                        ui.add(
                            egui::DragValue::new(&mut position.x)
                                .prefix("X:")
                                .speed(0.05),
                        );
                        ui.add(
                            egui::DragValue::new(&mut position.y)
                                .prefix("Y:")
                                .speed(0.05),
                        );
                    });
                    *position = round_vec2_to_tick(*position, GRANULARITY);
                });
            })
    }
}
