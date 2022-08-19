use crate::prelude::*;

pub fn auto_move(
    time: Res<Time>,
    mut query: Query<(&Movement, &mut AutoMove, &mut Transform)>,
) {
    for (movement, mut auto_move, mut transform) in query.iter_mut() {
        transform.rotate_z(movement.rotation_speed * time.delta_seconds());
    }
}