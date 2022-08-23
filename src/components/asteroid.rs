use std::fs::File;
use rand::prelude::{IteratorRandom, SliceRandom, ThreadRng};
use rand::Rng;
use ron::de::from_reader;
use crate::prelude::*;
use serde::Deserialize;


const INITIAL_SIZE: i32 = 3;

#[derive(Component)]
pub struct Asteroid {
    size: i32,
}

impl Asteroid {
    pub fn spawn(
        commands: &mut Commands,
        atlas_manager: &Res<AtlasManager>,
        spawn_point: Vec3,
    ) {
        let mut rng = rand::thread_rng();
        commands.spawn()
            .insert_bundle(
                SpriteSheetBundle {
                    texture_atlas: atlas_manager.texture_atlas.clone(),
                    transform: Transform::from_translation(spawn_point).with_scale(Vec3::splat(1.5)),
                    sprite: TextureAtlasSprite::new(atlas_manager.find_index("asteroid_large")),
                    ..Default::default()
                }
            )
            .insert(Asteroid {
                size: INITIAL_SIZE
            })
            .insert(Wrappable)
            .insert(AutoMove {
                direction: random_direction(&mut rng)
            })
            .insert(Movement {
                movement_speed: 150.0,
                rotation_speed: -f32::to_radians(45.0),
            });
    }

    pub fn split(
        &self,
        commands: &mut Commands,
        atlas_manager: &Res<AtlasManager>,
        asteroid_manager: &Res<AsteroidManager>,
        spawn_point: Vec3,
    ) {
        let new_size = self.size - 1;
        if new_size <= 0 {
            return;
        }
        let mut rng = rand::thread_rng();
        for _ in 0..2 {
            commands.spawn()
                .insert_bundle(
                    SpriteSheetBundle {
                        texture_atlas: atlas_manager.texture_atlas.clone(),
                        transform: Transform::from_translation(spawn_point.clone()).with_scale(Vec3::splat(1.5)),
                        sprite: TextureAtlasSprite::new(
                            atlas_manager.find_index(
                                asteroid_manager.random_asteroid(new_size)
                                    .expect("Could not find asteroid for given size"))
                        ),
                        ..Default::default()
                    }
                )
                .insert(Asteroid {
                    size: new_size
                })
                .insert(Wrappable)
                .insert(AutoMove {
                    direction: random_direction(&mut rng)
                })
                .insert(Movement {
                    movement_speed: 150.0,
                    rotation_speed: -f32::to_radians(45.0),
                });
        }
    }
}

fn random_direction(rng: &mut ThreadRng) -> Vec3 {
    let x = rng.gen_range(-1.0..1.0) as f32;
    let y = rng.gen_range(-1.0..1.0) as f32;
    Vec3::new(x, y, 0.0)
}

#[derive(Clone, Deserialize, Debug)]
pub struct AsteroidManager {
    pub asteroids: Vec<AsteroidResource>,
}

impl AsteroidManager {
    pub fn load_resource() -> Self {
        let file = File::open("assets/asteroids.ron").expect("Failed to open file: asteroids.ron");
        from_reader(file).expect("Could not deserialize asteroids.ron")
    }

    pub fn random_asteroid(&self, size: i32) -> Option<&String> {
        let mut rng = rand::thread_rng();
        self.asteroids.iter()
            .filter(|p| p.size == size)
            .flat_map(|p| &p.sprites)
            .choose(&mut rng)
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct AsteroidResource {
    size: i32,
    sprites: Vec<String>,
}