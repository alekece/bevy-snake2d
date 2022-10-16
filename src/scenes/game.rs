use std::f32::consts::PI;
use std::num::NonZeroUsize;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::Rng;

use crate::assets::Assets;
use crate::physics::{Direction, Position, PreviousDirection};
use crate::state::State;
use crate::systems;
use crate::world::apple::AppleSpawner;
use crate::world::tile_map::TileMap;
use crate::world::{snake, WorldPlugin};

const TILE_SIZE: f32 = 128.;

#[derive(Component)]
struct Player;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TileMap::new(8, 8))
            .insert_resource(AppleSpawner {
                timer: Timer::from_seconds(2.0, true),
                max_apples: 3,
            })
            .add_plugin(WorldPlugin)
            .add_enter_system(State::Game, setup_game)
            .add_system(handle_input.run_in_state(State::Game))
            .add_system_to_stage(CoreStage::PostUpdate, position_to_translation.run_in_state(State::Game))
            .add_exit_system(State::Game, systems::despawn_all);
    }
}

fn setup_game(mut commands: Commands, assets: Res<Assets>, tile_map: Res<TileMap>) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection { scale: 2., ..default() },
        ..default()
    });

    let entity = snake::spawn_snake(
        &mut commands,
        Direction::Down,
        IVec3::new(0, 0, 1),
        NonZeroUsize::new(3).unwrap(),
    );

    commands.entity(entity).insert(Player);

    let mut rng = rand::thread_rng();

    commands
        .spawn()
        .insert_bundle(TransformBundle::default())
        .insert_bundle(VisibilityBundle::default())
        .insert(Name::new("Tile Map"))
        .with_children(|parent| {
            tile_map.tiles().for_each(|tile| {
                parent
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(0.5, 0.9, 0.19),
                            custom_size: Some(Vec2::splat(TILE_SIZE)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Position(IVec3::new(tile.x() as i32, tile.y() as i32, 0)))
                    .insert(Name::new(format!("Tile ({}, {})", tile.x(), tile.y())));

                if rng.gen_bool(1. / 10.) && tile.is_empty() {
                    parent
                                .spawn_bundle(SpriteBundle {
                                    texture: assets.bush_texture.clone(),
                                    transform: Transform::from_rotation(Quat::from_rotation_z(
                                        rng.gen_range(0.0..180.) * PI / 180.,
                                    )).with_scale(Vec3::new(1., 1., 0.5)),
                                    //     .with_rotation(Quat::from_rotation_z(rng.gen_range(0.0..180.) * PI / 180.)),
                                    ..default()
                                })
                                // .insert(RigidBody)
                                .insert(Position(IVec3::new(tile.x() as i32, tile.y() as i32, 1)))
                                .insert(Name::new("Bush"));
                }
            });
        });
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

fn handle_input(
    mut commands: Commands,
    mut player: Query<(&mut Direction, &PreviousDirection), With<Player>>,
    mut camera: Query<&mut OrthographicProjection>,
    input: Res<Input<KeyCode>>,
) {
    let mut camera = camera.single_mut();

    if input.just_pressed(KeyCode::O) {
        camera.scale -= 0.1
    } else if input.just_pressed(KeyCode::P) {
        camera.scale += 0.1;
    } else if input.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(State::MainMenu))
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
