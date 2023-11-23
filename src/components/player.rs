use crate::prelude::*;

pub struct PlayerKilledEvent;

#[derive(Component)]
pub struct Player {
    fire_rate: f32,
    lives: i32,
}

impl Player {
    pub fn spawn(
        commands: &mut Commands,
        atlas_manager: &Res<AtlasManager>,
        spawn_point: Vec3,
    ) {
        commands.spawn()
            .insert_bundle(
                SpriteSheetBundle {
                    texture_atlas: atlas_manager.texture_atlas.clone(),
                    transform: Transform::from_translation(spawn_point),
                    sprite: TextureAtlasSprite::new(atlas_manager.find_index("ship_H")),
                    ..Default::default()
                })
            .insert(Player {
                fire_rate: 10.0,
                lives: 3,
            })
            .insert(Wrappable)
            .insert(Movement {
                movement_speed: 300.0,
                rotation_speed: f32::to_radians(180.0),
            });
    }

    pub fn loose_life(&mut self) {
        self.lives -= 1;
        self.lives = self.lives.max(0);
    }

    pub fn fire_rate(&self) -> f32 {
        1.0 / self.fire_rate
    }
}