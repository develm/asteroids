use rand::prelude::{SliceRandom, ThreadRng};
use rand::Rng;

use crate::prelude::*;

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
                    transform: Transform::from_translation(spawn_point).with_scale(Vec3::splat(INITIAL_SIZE as f32 / 2.0)),
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
        spawn_point: Vec3,
    ) {
        if self.size <= 0 {
            return;
        }
        let mut rng = rand::thread_rng();
        for _ in 0..2 {
            commands.spawn()
                .insert_bundle(
                    SpriteSheetBundle {
                        texture_atlas: atlas_manager.texture_atlas.clone(),
                        transform: Transform::from_translation(spawn_point.clone()).with_scale(Vec3::splat(self.size as f32 / 2.0)),
                        sprite: TextureAtlasSprite::new(atlas_manager.find_index("asteroid_half_3")),
                        ..Default::default()
                    }
                )
                .insert(Asteroid {
                    size: self.size - 1
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