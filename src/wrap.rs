use crate::{Expendable, Query, ResMut, Wrappable};
use crate::prelude::*;

pub fn wrap_window(
    windows: ResMut<Windows>,
    mut query: Query<&mut Transform, With<Wrappable>>,
) {
    let window = windows.get_primary().expect("Could not load window information");
    for mut transform in query.iter_mut() {
        transform.translation = wrap_coordinates(&transform.translation, window.width(), window.height());
    }
}

pub fn expend(
    mut commands: Commands,
    windows: ResMut<Windows>,
    query: Query<(Entity, &Transform), With<Expendable>>,
) {
    let window = windows.get_primary().expect("Could not load window information");
    for (e, transform) in query.iter() {
        if out_of_bounds(
            &transform.translation,
            window.width(),
            window.height(),
            50.0,
        ) {
            commands.entity(e).despawn();
        }
    }
}

fn out_of_bounds(cord: &Vec3, bound_x: f32, bound_y: f32, slack: f32) -> bool {
    cord.x < -(bound_x / 2.0 + slack)
        || cord.x > (bound_x / 2.0 + slack)
        || cord.y < -(bound_y / 2.0 + slack)
        || cord.y > (bound_y / 2.0 + slack)
}

fn wrap_coordinates(cord: &Vec3, bound_x: f32, bound_y: f32) -> Vec3 {
    let mut x = cord.x;
    let mut y = cord.y;
    if x < -(bound_x / 2.0) {
        x += bound_x;
    }
    if x > (bound_x / 2.0) {
        x -= bound_x;
    }
    if y < -(bound_y / 2.0) {
        y += bound_y;
    }
    if y > (bound_y / 2.0) {
        y -= bound_y;
    }
    Vec3::new(x, y, cord.z)
}