use bevy::sprite::Rect;
use crate::laser::Laser;
use crate::prelude::*;

pub struct GameSystems;

impl Plugin for GameSystems {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerKilledEvent>()
            .add_system(player_movement)
            .add_system(wrap_window)
            .add_system(expend)
            .add_system(auto_move)
            .add_system(player_action)
            .add_system(destroy_asteroid)
            .add_system(destroy_player)
            .add_system(lives_manager)
            .add_system(player_spawn_manager);
    }
}


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

pub fn player_action(
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
    asteroid_manager: Res<AsteroidManager>,
    atlases: ResMut<Assets<TextureAtlas>>,
    lasers: Query<(Entity, &Transform), With<Laser>>,
    asteroids: Query<(Entity, &Asteroid, &TextureAtlasSprite, &Transform)>,
) {
    let atlas = atlases.get(&atlas_manager.texture_atlas).expect("Texture atlas not found");
    for (l_entity, laser) in lasers.iter() {
        for (a_entity, asteroid, sprite, a_position) in asteroids.iter() {
            // sprite size
            let size = atlas.textures.get(sprite.index).expect("Texture size not found");
            if in_bounds(laser.translation, a_position.translation, size.width(), size.height()) {
                // despawn asteroid and laser, spawn two smaller asteroids
                asteroid.split(&mut commands, &atlas_manager, &asteroid_manager, a_position.translation);
                commands.entity(l_entity).despawn();
                commands.entity(a_entity).despawn();
            }
        }
    }
}

pub fn destroy_player(
    atlas_manager: Res<AtlasManager>,
    atlases: ResMut<Assets<TextureAtlas>>,
    mut ev_player: EventWriter<PlayerKilledEvent>,
    player: Query<&Transform, With<Player>>,
    asteroids: Query<(&TextureAtlasSprite, &Transform), With<Asteroid>>,
) {
    let  p_position = match player.get_single() {
        Ok(q) => q,
        Err(_) => return
    };
    let atlas = atlases.get(&atlas_manager.texture_atlas).expect("Texture atlas not found");
    for (sprite, asteroid) in asteroids.iter() {
        let size = atlas.textures.get(sprite.index).expect("Texture size not found");
        if in_bounds(p_position.translation, asteroid.translation, size.width(), size.height()) {
            ev_player.send(PlayerKilledEvent);
        }
    }
}

pub fn lives_manager(
    mut player_query: Query<&mut Player>,
    mut events: EventReader<PlayerKilledEvent>,
) {
    let mut player = match player_query.get_single_mut() {
        Ok(q) => q,
        Err(_) => return
    };

    for _ in events.iter() {
        player.loose_life();
    }
}

pub fn player_spawn_manager(
    atlas_manager: Res<AtlasManager>,
    windows: ResMut<Windows>,
    atlases: ResMut<Assets<TextureAtlas>>,
    events: EventReader<PlayerKilledEvent>,
    mut player_query: Query<&mut Transform, With<Player>>,
    asteroid_query: Query<(&Asteroid, &TextureAtlasSprite, &Transform), Without<Player>>,
) {
    if events.is_empty() {
        return;
    }
    let mut player_position = match player_query.get_single_mut() {
        Ok(q) => q,
        Err(_) => return
    };
    let atlas = atlases.get(&atlas_manager.texture_atlas).expect("Texture atlas not found");
    let mut asteroid_positions: Vec<(Vec3, Rect)> = Vec::new();
    for (_, sprite, a_position) in asteroid_query.iter() {
        let size: &Rect = atlas.textures.get(sprite.index).expect("Texture size not found");
        asteroid_positions.push((a_position.translation, *size));
    }

    let window = windows.get_primary().expect("Could not load window information");
    let mut rng = rand::thread_rng();
    player_position.translation = random_position(&mut rng, Vec2::new(window.width(), window.height()), &asteroid_positions);


}

