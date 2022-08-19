use crate::prelude::*;

#[derive(Component)]
pub struct Asteroid;

impl Asteroid {
    pub fn spawn(
        commands: &mut Commands,
        atlas_manager: &Res<AtlasManager>,
        spawn_point: Vec3
    ) {
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
                direction: Vec3::splat(0.0)
            })
            .insert(Movement {
                movement_speed: 250.0,
                rotation_speed: -f32::to_radians(45.0),
            });
    }
}