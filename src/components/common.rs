use crate::prelude::*;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct Movement {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct Wrappable;

#[derive(Component)]
pub struct Expendable;