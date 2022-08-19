use crate::prelude::*;

pub fn auto_move(
    time: Res<Time>,
    mut query: Query<(&Movement, &AutoMove, &mut Transform)>,
) {
    for (movement, auto_move, mut transform) in query.iter_mut() {
        transform.rotate_z(movement.rotation_speed * time.delta_seconds());
        transform.translation += auto_move.direction * movement.movement_speed * time.delta_seconds();
    }
}