use bevy::prelude::*;
use crate::Player;
use crate::util::wrap_coordinates;

pub fn player_movement(
    key: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: ResMut<Windows>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let window = windows.get_primary().expect("Could not load window information");
    let (player, mut transform) = query.single_mut();
    // Handle Rotation
    let mut rotation_factor = 0.0;
    if key.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    } else if key.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    // Handle Movement
    let mut movement_factor = 0.0;
    if key.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    transform.rotate_z(rotation_factor * player.rotation_speed * time.delta_seconds());

    let direction = transform.rotation * -Vec3::X;
    let distance = movement_factor * player.movement_speed * time.delta_seconds();
    let translation_delta = direction * distance;
    transform.translation += translation_delta;
    transform.translation = wrap_coordinates(&transform.translation, window.width(), window.height());
}