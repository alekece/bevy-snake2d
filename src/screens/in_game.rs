use std::num::NonZeroUsize;

use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::assets::TextureAssets;
use crate::game::apple::AppleSpawner;
use crate::game::bush::BushBuilder;
use crate::game::components::{Direction, Lifetime, NumberGenerator, Position};
use crate::game::snake::{SnakeAction, SnakeEvent};
use crate::game::tile_map::{self, TileMap, TileMapOptions, TileMapPosition};
use crate::game::wind::WindTimer;
use crate::game::{bush, snake, GamePlugin};
use crate::states::AppScreen;
use crate::systems;

#[derive(Component)]
struct Player;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GamePlugin)
            .add_enter_system(AppScreen::InGame, setup_game)
            .add_exit_system_set(
                AppScreen::InGame,
                SystemSet::new()
                    .with_system(systems::despawn_all)
                    .with_system(systems::despawn_resource::<TileMap>)
                    .with_system(systems::despawn_resource::<AppleSpawner>),
            );
    }
}

fn setup_game(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.5,
            ..default()
        },
        ..default()
    });

    let tile_map = TileMap::empty(16, 9);

    let entity = snake::spawn_snake(
        &mut commands,
        Direction::Down,
        IVec3::new(4, 5, 1),
        NonZeroUsize::new(4).unwrap(),
        NumberGenerator::from_range(0..7),
    );

    for (x, y, angle) in [(5, 5, 50.0), (12, 8, 0.0), (1, 0, 170.0), (2, 4, 234.0)] {
        BushBuilder::default()
            .with_position(IVec3::new(x, y, 1))
            .with_angle(angle)
            // .animate()
            .spawn(&mut commands, &textures);
    }

    commands.insert_resource(WindTimer::new(5.0..10.0));

    commands
        .entity(entity)
        .insert_bundle(InputManagerBundle::<SnakeAction> {
            input_map: InputMap::new([
                (KeyCode::Up, SnakeAction::MoveUp),
                (KeyCode::Left, SnakeAction::MoveLeft),
                (KeyCode::Right, SnakeAction::MoveRight),
                (KeyCode::Down, SnakeAction::MoveDown),
            ]),
            ..Default::default()
        })
        .insert(Player);

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
                            custom_size: Some(Vec2::splat(tile_map::TILE_SIZE)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Position(IVec3::new(tile.x() as i32, tile.y() as i32, 0)))
                    .insert(Name::new(format!("Tile ({}, {})", tile.x(), tile.y())));
            });
        });

    commands.insert_resource(tile_map);
    commands.insert_resource(TileMapOptions {
        tile_size: 128.0,
        position: TileMapPosition::Centered,
    });

    commands.insert_resource(AppleSpawner {
        timer: Timer::from_seconds(2.0, true),
        max_apples: 3,
        apple_lifetime: Some(Lifetime::from_seconds(5.0)),
    });
}
