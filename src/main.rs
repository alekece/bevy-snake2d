#![allow(incomplete_features)]
#![feature(adt_const_params)]

mod assets;
mod scenes;
mod state;
mod systems;
mod physics;
mod world;

use bevy::prelude::*;
use bevy::window::PresentMode;
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use scenes::ScenesPlugin;
use assets::Assets;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: format!("Bevy Snake 2D v{VERSION}"),
        present_mode: PresentMode::AutoVsync,
        width: 1920.,
        height: 1080.,
        resizable: false,
        ..Default::default()
    })
    .insert_resource(Msaa { samples: 4 })
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins)
    .init_resource::<Assets>()
    .add_plugin(ScenesPlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.run();
}


