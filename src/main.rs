#![warn(clippy::pedantic)]
#![allow(
    clippy::type_complexity,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::cast_possible_wrap, // TODO
    clippy::cast_possible_truncation, // TODO
    clippy::cast_precision_loss, // TODO
)]
#![allow(dead_code)] // TODO

mod assets;
mod game;
mod screens;
mod states;
mod systems;
mod run_criterias;

use bevy::prelude::*;
use bevy::window::PresentMode;
#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

use screens::ScreensPlugin;

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
    .add_plugins_with(DefaultPlugins, |plugins| {
        #[cfg(feature = "print-schedule")]
        plugins.disable::<bevy::log::LogPlugin>();

        plugins
    })
    .add_plugin(ScreensPlugin);

    #[cfg(feature = "debug")]
    app.add_plugin(WorldInspectorPlugin::new());

    cfg_if::cfg_if! {
        if #[cfg(feature = "print-schedule")] {
            bevy_mod_debugdump::print_schedule(&mut app);
        } else {
            app.run();
        }
    }
}
