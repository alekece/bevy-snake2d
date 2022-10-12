mod asset_server;
mod assets;
mod scenes;
mod state;
mod systems;

// mod menu;
// mod assets;

// const VERSION: &str = env!("CARGO_PGK_VERSION");

// fn main() {
//     let mut app = App::new();

//     app.insert_resource(WindowDescriptor {
//         title: format!("Bevy Snake 2D v{}", VERSION),
//         present_mode: PresentMode::AutoVsync,
//         width: 1920.,
//         height: 1080.,
//         resizable: false,
//         ..Default::default()
//     })
//     .insert_resource(Msaa { samples: 4 })
//     .insert_resource(ImageSettings::default_nearest())
//     .insert_resource(ClearColor(BACKGROUND_COLOR))
//     .add_plugins(DefaultPlugins)
//     .add_plugins(menu::SplashPlugin);
//     // .add_plugins(menu::MainPlugin);
//     // .add_plugins(menu::SettingsPlugin);
//     // .add_plugins(menu::GamePlugin);

//     #[cfg(feature = "debug")]
//     {
//         use bevy_inspector_egui::WorldInspectorPlugin;

//         app.add_plugin(WorldInspectorPlugin::new());
//     }

//     app.run();
// }

use std::num::NonZeroUsize;

use apple::ApplePlugin;
use assets::AssetsPlugin;
use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::window::PresentMode;

use board::BoardPlugin;
use scenes::ScenePlugins;
use snake::{
    render_snake,
    spawn_snake,
    Direction,
    Position,
    PreviousDirection,
    Size,
    SnakeFragment,
    SnakeHead,
    SnakePlugin,
};
use state::StatePlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
const TILE_SIZE: f32 = 128.;

pub struct SnakeFragmentAssets {
    fragment_texture: Handle<Image>,
    fragment_right_texture: Handle<Image>,
    fragment_left_texture: Handle<Image>,
}

pub struct GameAssets {
    apple_texture: Handle<Image>,
    snake_head_texture: Handle<Image>,
    snake_fragment_assets: Vec<SnakeFragmentAssets>,
    snake_tail_texture: Handle<Image>,
    box_texture: Handle<Image>,
    bush_texture: Handle<Image>,
}

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let snake_fragment_assets = (1..=6)
            .into_iter()
            .map(|i| {
                SnakeFragmentAssets {
                    fragment_texture: asset_server.load(&format!("sprites/min-128x128/snake_fragment_{i}.png")),
                    fragment_right_texture: asset_server
                        .load(&format!("sprites/min-128x128/snake_fragment_right_{i}.png")),
                    fragment_left_texture: asset_server
                        .load(&format!("sprites/min-128x128/snake_fragment_left_{i}.png")),
                }
            })
            .collect();

        Self {
            snake_fragment_assets,
            apple_texture: asset_server.load("sprites/min-128x128/apple.png"),
            snake_head_texture: asset_server.load("sprites/min-128x128/snake_head.png"),
            snake_tail_texture: asset_server.load("sprites/min-128x128/snake_tail.png"),
            box_texture: asset_server.load("sprites/min-128x128/box.png"),
            bush_texture: asset_server.load("sprites/min-128x128/bush.png"),
        }
    }
}

#[derive(StageLabel)]
pub enum GameStage {
    PreRender,
}

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Bevy Snake 2D".to_string(),
        present_mode: PresentMode::AutoVsync,
        width: 1920.,
        height: 1180.,
        resizable: false,
        ..Default::default()
    })
    .insert_resource(Msaa { samples: 4 })
    // .insert_resource(ImageSettings::default_nearest())
    .insert_resource(ClearColor(BACKGROUND_COLOR))
    .add_plugins(DefaultPlugins)
    .add_stage_before(CoreStage::PostUpdate, GameStage::PreRender, SystemStage::parallel())
    .init_resource::<GameAssets>()
    .add_plugin(SnakePlugin)
    .add_plugin(ApplePlugin)
    .add_plugin(BoardPlugin)
    // .add_plugin(StatePlugin)
    // .add_plugin(AssetsPlugin)
    // .add_plugins(ScenePlugins)
    .add_system(bevy::window::close_on_esc)
    .add_startup_system(setup)
    .add_system(handle_input)
    .add_system_set_to_stage(
        GameStage::PreRender,
        SystemSet::new()
            .with_system(position_to_translation.after(render_snake))
            .with_system(size_to_scale.after(position_to_translation)),
    );

    #[cfg(feature = "debug")]
    {
        use bevy_inspector_egui::WorldInspectorPlugin;

        app.register_type::<Player>().add_plugin(WorldInspectorPlugin::new());
    }

    app.run();
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 2.,
            ..Default::default()
        },
        ..Default::default()
    });

    let entity = spawn_snake(
        &mut commands,
        Direction::Down,
        IVec3::new(0, 0, 1),
        NonZeroUsize::new(3).unwrap(),
    );

    commands.entity(entity).insert(Player);
}

fn handle_input(
    mut player: Query<(&mut Direction, &PreviousDirection), With<Player>>,
    mut camera: Query<&mut OrthographicProjection>,
    input: Res<Input<KeyCode>>,
) {
    let mut camera = camera.single_mut();

    if input.just_pressed(KeyCode::O) {
        camera.scale -= 0.1
    } else if input.just_pressed(KeyCode::P) {
        camera.scale += 0.1;
    }

    let new_direction = if input.pressed(KeyCode::Up) {
        Direction::Up
    } else if input.pressed(KeyCode::Down) {
        Direction::Down
    } else if input.pressed(KeyCode::Left) {
        Direction::Left
    } else if input.pressed(KeyCode::Right) {
        Direction::Right
    } else {
        return;
    };

    if let Ok((mut direction, previous_direction)) = player.get_single_mut() {
        if new_direction != previous_direction.0.opposite() {
            *direction = new_direction;
        }
    }
}

fn position_to_translation(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation = Vec3::new(
            position.0.x as f32 * TILE_SIZE,
            position.0.y as f32 * TILE_SIZE,
            position.0.z as f32,
        );
    }
}

fn size_to_scale(mut query: Query<(&mut Transform, &Size)>) {
    for (mut transform, size) in query.iter_mut() {
        transform.scale = Vec3::new(size.0.x * TILE_SIZE, size.0.y * TILE_SIZE, 1.);
    }
}
