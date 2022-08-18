use bevy::prelude::{Transform, Vec3, Windows};
use crate::{Query, ResMut};

pub fn wrap_window(
    windows: ResMut<Windows>,
    mut query: Query<&mut Transform>,
) {
    let window = windows.get_primary().expect("Could not load window information");
    for mut transform in query.iter_mut() {
        transform.translation = wrap_coordinates(&transform.translation, window.width(), window.height());
    }
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