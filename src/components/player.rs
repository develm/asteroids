use crate::prelude::*;

#[derive(Component)]
pub struct Player {
    fire_rate: f32,
}

impl Player {
    pub fn spawn(
        commands: &mut Commands,
        atlas_manager: &Res<AtlasManager>,
        spawn_point: Vec3
    ) {
        commands.spawn()
            .insert_bundle(
                SpriteSheetBundle {
                    texture_atlas: atlas_manager.texture_atlas.clone(),
                    transform: Transform::from_translation(spawn_point).with_scale(ASSET_SCALING),
                    sprite: TextureAtlasSprite::new(atlas_manager.find_index("spaceship")),
                    ..Default::default()
                })
            .insert(Player {
                fire_rate: 10.0
            })
            .insert(Wrappable)
            .insert(Movement {
                movement_speed: 300.0,
                rotation_speed: f32::to_radians(180.0),
            });
    }

    pub fn fire_rate(&self) -> f32 {
        1.0 / self.fire_rate
    }
}