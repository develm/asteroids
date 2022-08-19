use rand::prelude::ThreadRng;
use rand::Rng;
use crate::prelude::*;

#[derive(Component)]
pub struct Asteroid;

impl Asteroid {
    pub fn spawn(
        commands: &mut Commands,
        atlas_manager: &Res<AtlasManager>,
        spawn_point: Vec3
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
            .insert(Asteroid)
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

fn random_direction(rng: &mut ThreadRng) -> Vec3 {
    let x = rng.gen_range(-1.0..1.0) as f32;
    let y = rng.gen_range(-1.0..1.0) as f32;
    Vec3::new(x, y, 0.0)
}