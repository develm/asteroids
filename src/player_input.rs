use bevy::prelude::*;
use crate::Player;

pub fn handle_player_input(
    key: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
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
}