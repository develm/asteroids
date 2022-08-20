use crate::laser::Laser;
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

pub fn player_movement(
    key: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Movement, &mut Transform), With<Player>>,
) {
    let (movement, mut transform) = match query.get_single_mut() {
        Ok(q) => q,
        Err(_) => return
    };
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

    transform.rotate_z(rotation_factor * movement.rotation_speed * time.delta_seconds());

    let direction = transform.rotation * -Vec3::X;
    let distance = movement_factor * movement.movement_speed * time.delta_seconds();
    let translation_delta = direction * distance;
    transform.translation += translation_delta;
}

pub fn player_action (
    mut commands: Commands,
    key: Res<Input<KeyCode>>,
    atlas_manager: Res<AtlasManager>,
    time: Res<Time>,
    mut elapsed: Local<f32>,
    query: Query<(&Player, &Transform)>,
) {
    let (player, position) = match query.get_single() {
        Ok(q) => q,
        Err(_) => return
    };
    *elapsed += time.delta_seconds();
    if key.just_pressed(KeyCode::Space) && *elapsed > player.fire_rate() {
        let direction = position.rotation * -Vec3::X;
        let position = position.translation + (Vec3::new(20.0, 20.0, -1.0) * direction);
        Laser::spawn(&mut commands, atlas_manager, position, direction);
        *elapsed = 0.0;
    }
}

pub fn wrap_window(
    windows: ResMut<Windows>,
    mut query: Query<&mut Transform, With<Wrappable>>,
) {
    let window = windows.get_primary().expect("Could not load window information");
    for mut transform in query.iter_mut() {
        transform.translation = wrap_coordinates(transform.translation, window.width(), window.height());
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
            transform.translation,
            window.width(),
            window.height(),
            50.0,
        ) {
            commands.entity(e).despawn();
        }
    }
}

pub fn destroy_asteroid(
    mut commands: Commands,
    atlas_manager: Res<AtlasManager>,
    atlases: ResMut<Assets<TextureAtlas>>,
    lasers: Query<(Entity, &Transform), With<Laser>>,
    asteroids: Query<(Entity, &TextureAtlasSprite, &Transform), With<Asteroid>>,
) {
    let atlas = atlases.get(&atlas_manager.texture_atlas).expect("Texture atlas not found");
    for (le, laser) in lasers.iter() {
        for (ae, sprite, asteroid) in asteroids.iter() {
            // sprite size
            let size = atlas.textures.get(sprite.index).expect("Texture size not found");
            if in_bounds(laser.translation, asteroid.translation, size.width(), size.height()) {
                // despawn asteroid and laser
                commands.entity(le).despawn();
                commands.entity(ae).despawn();
            }

        }
    }
}

pub fn destroy_player (
    mut commands: Commands,
    atlas_manager: Res<AtlasManager>,
    atlases: ResMut<Assets<TextureAtlas>>,
    player: Query<(Entity, &Transform), With<Player>>,
    asteroids: Query<(&TextureAtlasSprite, &Transform), With<Asteroid>>,
) {
    let (pe, player) = match player.get_single() {
        Ok(q) => q,
        Err(_) => return
    };
    let atlas = atlases.get(&atlas_manager.texture_atlas).expect("Texture atlas not found");
    for (sprite, asteroid) in asteroids.iter() {
        let size = atlas.textures.get(sprite.index).expect("Texture size not found");
        if in_bounds(player.translation, asteroid.translation, size.width(), size.height()) {
            commands.entity(pe).despawn();
        }
    }
}