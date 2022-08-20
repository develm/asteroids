use crate::prelude::*;

pub fn out_of_bounds(cord: &Vec3, bound_x: f32, bound_y: f32, slack: f32) -> bool {
    cord.x < -(bound_x / 2.0 + slack)
        || cord.x > (bound_x / 2.0 + slack)
        || cord.y < -(bound_y / 2.0 + slack)
        || cord.y > (bound_y / 2.0 + slack)
}

pub fn wrap_coordinates(cord: &Vec3, bound_x: f32, bound_y: f32) -> Vec3 {
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