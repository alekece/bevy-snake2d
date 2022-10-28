pub mod in_game;
pub mod main_menu;
pub mod splash;

use bevy::prelude::*;
use bevy_tweening::TweeningPlugin;
use iyes_loopless::prelude::*;

use crate::assets::{FontAssets, TextureAssets};
use crate::states::AppScreen;

use in_game::InGamePlugin;
use main_menu::MainMenuPlugin;
use splash::SplashPlugin;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(AppScreen::InGame)
            .init_resource::<FontAssets>()
            .init_resource::<TextureAssets>()
            .add_plugin(TweeningPlugin)
            .add_plugin(SplashPlugin)
            .add_plugin(MainMenuPlugin)
            .add_plugin(InGamePlugin);
    }
}
