pub mod snake;
pub mod apple;
pub mod tile_map;

use bevy::prelude::*;

use apple::ApplePlugin;
use snake::SnakePlugin;
use tile_map::TileMapPlugin;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ApplePlugin)
            .add_plugin(SnakePlugin)
            .add_plugin(TileMapPlugin);
    }
}
