use crate::prelude::*;

#[derive(Component)]
pub struct Laser;


impl Laser {
    pub fn spawn(
        commands: &mut Commands,
        atlas_manager: Res<AtlasManager>,
        spawn_point: Vec3,
        direction: Vec3,
    ) {
        commands.spawn()
            .insert_bundle(
                SpriteSheetBundle {
                    texture_atlas: atlas_manager.texture_atlas.clone(),
                    transform: Transform::from_translation(spawn_point),
                    sprite: TextureAtlasSprite::new(atlas_manager.find_index("star_small")),
                    ..Default::default()
                }
            )
            .insert(Laser)
            .insert(Expendable)
            .insert(Movement {
                movement_speed: 600.0,
                rotation_speed: 0.0,
            })
            .insert(AutoMove {
                direction
            });
    }
}