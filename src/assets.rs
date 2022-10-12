use bevy::prelude::*;

pub trait AssetServerExt {
    fn load_sprite(&self, sprite: &str) -> Handle<Image>;
    fn load_font(&self, sprite: &str) -> Handle<Font>;
}

impl AssetServerExt for AssetServer {
    fn load_sprite(&self, sprite: &str) -> Handle<Image> {
        self.load(&format!("sprites/min-128x128/{sprite}"))
    }

    fn load_font(&self, font: &str) -> Handle<Font> {
        self.load(&format!("fonts/{font}"))
    }
}

pub struct SnakeFragmentAssets {
    pub straight_texture: Handle<Image>,
    pub right_curved_texture: Handle<Image>,
    pub left_curved_texture: Handle<Image>,
}

pub struct Assets {
    pub apple_texture: Handle<Image>,
    pub snake_head_texture: Handle<Image>,
    pub snake_tail_texture: Handle<Image>,
    pub snake_fragment_assets: Vec<SnakeFragmentAssets>,
    pub box_texture: Handle<Image>,
    pub bush_texture: Handle<Image>,
    pub title_font: Handle<Font>,
    pub headline_font: Handle<Font>,
    pub blurry_texture: Handle<Image>,
}

impl FromWorld for Assets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let snake_fragment_assets = (1..=6)
            .into_iter()
            .map(|i| {
                SnakeFragmentAssets {
                    straight_texture: asset_server.load_sprite(&format!("snake_fragment_{i}.png")),
                    right_curved_texture: asset_server.load_sprite(&format!("snake_fragment_right_{i}.png")),
                    left_curved_texture: asset_server.load_sprite(&format!("snake_fragment_left_{i}.png")),
                }
            })
            .collect();

        Self {
            snake_fragment_assets,
            apple_texture: asset_server.load_sprite("apple.png"),
            snake_head_texture: asset_server.load_sprite("snake_head.png"),
            snake_tail_texture: asset_server.load_sprite("snake_tail.png"),
            box_texture: asset_server.load_sprite("box.png"),
            bush_texture: asset_server.load_sprite("bush.png"),
            title_font: asset_server.load_font("blomberg.otf"),
            headline_font: asset_server.load_font("emotion_engine.ttf"),
            blurry_texture: asset_server.load_sprite("blurry.png"),
        }
    }
}
