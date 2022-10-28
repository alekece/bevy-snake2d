pub mod apple;
pub mod components;
// pub mod collision;
pub mod snake;
pub mod tile_map;
pub mod value_tracker;
pub mod queries;
pub mod bush;
pub mod wind;

use bevy::prelude::*;

use apple::ApplePlugin;
use snake::SnakePlugin;
use tile_map::TileMapPlugin;
use value_tracker::ValueTrackerPlugin;
use components::Direction;
use wind::WindPlugin;

#[derive(StageLabel)]
pub enum GameStage {
    Track,
    Transform,
    Cleanup,
}

#[derive(SystemLabel)]
pub enum GameSystem {
    Movement,
    CheckPosition,
    CollisionDetection,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(CoreStage::Update, GameStage::Track, SystemStage::parallel())
            .add_stage_after(GameStage::Track, GameStage::Transform, SystemStage::parallel())
            .add_stage_after(GameStage::Transform, GameStage::Cleanup, SystemStage::parallel())
            .add_plugin(ValueTrackerPlugin::<Direction>::default())
            .add_plugin(ApplePlugin)
            .add_plugin(SnakePlugin)
            .add_plugin(WindPlugin)
            .add_plugin(TileMapPlugin);
        // .add_plugin(CollisionPlugin);
    }
}
