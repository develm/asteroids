use std::fs::File;
use ron::de::from_reader;
use bevy::prelude::*;
use bevy::sprite::Rect;
use bevy::utils::HashMap;
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
struct Sprite {
    name: String,
    position: (f32, f32, f32, f32),
}

#[derive(Clone, Deserialize, Debug)]
struct SpriteSheet {
    width: f32,
    height: f32,
    sprites: Vec<Sprite>,
}

impl SpriteSheet {
    fn load() -> Self {
        let file = File::open("assets/atlas.ron").expect("Failed to open atlas.ron file");
        from_reader(file).expect("Could not deserialize atlas.ron")
    }
}

#[derive(Clone, Default, Debug)]
pub struct AtlasManager {
    pub texture_atlas: Handle<TextureAtlas>,
    pub texture_index: HashMap<String, i32>,
}

pub struct AssetManagerPlugin;

impl Plugin for AssetManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<AtlasManager>()
            .add_startup_system(AssetManagerPlugin::load_assets);
    }
}

impl AssetManagerPlugin {
    fn load_assets(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>
    ) {
        let sprite_sheet = SpriteSheet::load();
        let texture_handle = asset_server.load("atlas.png");


        let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(sprite_sheet.width, sprite_sheet.height));

        let mut texture_index = HashMap::new();
        for (i, sprite) in sprite_sheet.sprites.iter().enumerate() {
            texture_index.insert(sprite.name.clone(), i as i32);
            let (x, y, w, h) = sprite.position;
            texture_atlas.add_texture(
                Rect {
                    min: Vec2::new(x, y),
                    max: Vec2::new(w, h),
                });
        }


        let handle= texture_atlases.add(texture_atlas);
        commands.insert_resource(AtlasManager{
            texture_atlas: handle,
            texture_index,
        });
    }
}


