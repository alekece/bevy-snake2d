use bevy::prelude::*;

pub trait AssetServerExt {
    fn load_texture(&self, texture: &str) -> Handle<Image>;
    fn load_font(&self, texture: &str) -> Handle<Font>;
}

impl AssetServerExt for AssetServer {
    fn load_texture(&self, texture: &str) -> Handle<Image> {
        self.load(&format!("textures/{texture}"))
    }

    fn load_font(&self, font: &str) -> Handle<Font> {
        self.load(&format!("fonts/{font}"))
    }
}

pub struct FontAssets {
    pub text: Handle<Font>,
}

impl FromWorld for FontAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        Self {
            text: asset_server.load_font("blomberg.otf"),
        }
    }
}

pub struct SnakeFragmentTextureAssets {
    pub straight: Handle<Image>,
    pub right_curved: Handle<Image>,
    pub left_curved: Handle<Image>,
}

pub struct TextureAssets {
    pub apple: Handle<Image>,
    pub apple_leaf: Handle<Image>,
    pub snake_head: Handle<Image>,
    pub snake_tail: Handle<Image>,
    pub snake_fragment_assets: Vec<SnakeFragmentTextureAssets>,
    pub bush_lower: Handle<Image>,
    pub bush_upper: Handle<Image>,
}

impl FromWorld for TextureAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let snake_fragment_assets = (1..=7)
            .into_iter()
            .map(|i| SnakeFragmentTextureAssets {
                straight: asset_server.load_texture(&format!("snake_fragment_{i}.png")),
                right_curved: asset_server.load_texture(&format!("snake_fragment_right_{i}.png")),
                left_curved: asset_server.load_texture(&format!("snake_fragment_left_{i}.png")),
            })
            .collect();

        Self {
            snake_fragment_assets,
            apple: asset_server.load_texture("apple.png"),
            apple_leaf: asset_server.load_texture("apple_leaf.png"),
            snake_head: asset_server.load_texture("snake_head.png"),
            snake_tail: asset_server.load_texture("snake_tail.png"),
            bush_lower: asset_server.load_texture("bush_lower.png"),
            bush_upper: asset_server.load_texture("bush_upper.png"),
        }
    }
}
