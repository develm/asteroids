use crate::prelude::*;

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn spawn(commands: &mut Commands, atlas_manager: &Res<AtlasManager>, spawn_point: Vec3) {
        commands.spawn()
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: atlas_manager.texture_atlas.clone(),
                transform: Transform::from_translation(spawn_point).with_scale(SCALE),
                sprite: TextureAtlasSprite::new(atlas_manager.find_index("spaceship")),
                ..Default::default()
            })
            .insert(Player)
            .insert(Wrappable)
            .insert(Movement {
                movement_speed: 300.0,
                rotation_speed: f32::to_radians(180.0),
            });
    }
}