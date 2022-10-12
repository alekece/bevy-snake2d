pub mod splash;
pub mod main_menu;
pub mod game;

use bevy::prelude::*;

use splash::SplashPlugin;
use main_menu::MainMenuPlugin;
use game::GamePlugin;

pub struct ScenesPlugin;

impl Plugin for ScenesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(SplashPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(GamePlugin);
    }
}
