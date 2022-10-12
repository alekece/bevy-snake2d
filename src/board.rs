use std::f32::consts::PI;
use std::marker::PhantomData;

use bevy::app::AppExit;
use bevy::prelude::*;
use rand::Rng;

use crate::apple::Apple;
use crate::snake::{Position, Size, SnakeFragment, SnakeHead};
use crate::GameAssets;

const BOARD_TILE_COLOR: Color = Color::rgb(0.5, 0.9, 0.19);
const BOARD_TILE_SIZE: Vec2 = Vec2::splat(0.99);

#[derive(Component)]
pub struct RigidBody;

pub struct CollisionEvent<T, U> {
    pub collider: Entity,
    pub collided: Entity,
    _marker: PhantomData<(T, U)>,
}

impl<T, U> CollisionEvent<T, U> {
    pub fn new(collider: Entity, collided: Entity) -> Self {
        Self {
            collided,
            collider,
            _marker: PhantomData,
        }
    }
}

pub struct Board {
    pub size: IVec2,
}

impl Default for Board {
    fn default() -> Self {
        Self { size: IVec2::splat(12) }
    }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_startup_system(spawn_board)
            .add_event::<CollisionEvent<SnakeHead, Apple>>()
            .add_event::<CollisionEvent<SnakeHead, RigidBody>>()
            .add_event::<CollisionEvent<SnakeHead, SnakeFragment>>()
            .add_system(game_over::<SnakeFragment>)
            .add_system(game_over::<RigidBody>)
            .add_system(clamp_position.after("move"))
            .add_system(detect_collision::<SnakeHead, Apple>.after("move"))
            .add_system(detect_collision::<SnakeHead, SnakeFragment>.after("move"))
            .add_system(detect_collision::<SnakeHead, RigidBody>.after("move"));
    }
}

fn game_over<T: Component>(
    collision_reader: EventReader<CollisionEvent<SnakeHead, T>>,
    mut exit: EventWriter<AppExit>,
) {
    if !collision_reader.is_empty() {
        exit.send(AppExit);
    }
}

pub fn detect_collision<T, U>(
    mut collision_writer: EventWriter<CollisionEvent<T, U>>,
    moved_entities: Query<(Entity, &Position), (With<T>, Changed<Position>)>,
    entities: Query<(Entity, &Position), (Without<T>, With<U>)>,
) where
    T: Component,
    U: Component,
{
    moved_entities.for_each(|(moved_entity, moved_position)| {
        entities.for_each(|(entity, position)| {
            if moved_entity != entity && moved_position == position {
                collision_writer.send(CollisionEvent::new(moved_entity, entity));
            }
        });
    });
}

fn clamp_position(board: Res<Board>, mut positions: Query<&mut Position>) {
    for mut position in positions.iter_mut() {
        let x_bound = board.size.x / 2;
        let y_bound = board.size.y / 2;

        if position.0.x >= x_bound {
            position.0.x -= board.size.x;
        } else if position.0.x < -x_bound {
            position.0.x += board.size.x;
        }
        if position.0.y >= y_bound {
            position.0.y -= board.size.y;
        } else if position.0.y < -y_bound {
            position.0.y += board.size.y;
        }
    }
}

fn spawn_board(mut commands: Commands, game_assets: Res<GameAssets>, board: Res<Board>) {
    let x_bound = board.size.x / 2;
    let y_bound = board.size.y / 2;
    let mut rng = rand::thread_rng();

    commands
        .spawn()
        .insert_bundle(TransformBundle::default())
        .insert_bundle(VisibilityBundle::default())
        .insert(Name::new("Board"))
        .with_children(|parent| {
            for y in -y_bound..y_bound {
                for x in -x_bound..x_bound {
                    parent
                        .spawn_bundle(SpriteBundle {
                            sprite: Sprite {
                                color: BOARD_TILE_COLOR,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Size(BOARD_TILE_SIZE))
                        .insert(Position(IVec3::new(x, y, 0)))
                        .insert(Name::new(format!("Tile ({x}, {y})")));

                    // if x == -x_bound || x == x_bound - 1 || y == -y_bound || y == y_bound - 1 {
                    //     parent
                    //         .spawn_bundle(SpriteBundle {
                    //             texture: game_assets.box_texture.clone(),
                    //             ..Default::default()
                    //         })
                    //         .insert(Position(IVec3::new(x, y, 1)))
                    //         .insert(RigidBody)
                    //         .insert(Name::new("Box"));
                    // }
                    if rng.gen_bool(1. / 10.) {
                        parent
                            .spawn_bundle(SpriteBundle {
                                texture: game_assets.bush_texture.clone(),
                                transform: Transform::from_rotation(Quat::from_rotation_z(
                                    rng.gen_range(0.0..180.) * PI / 180.,
                                )).with_scale(Vec3::new(1., 1., 0.5)),
                                //     .with_rotation(Quat::from_rotation_z(rng.gen_range(0.0..180.) * PI / 180.)),
                                ..Default::default()
                            })
                            // .insert(RigidBody)
                            .insert(Position(IVec3::new(x, y, 1)))
                            .insert(Name::new("Bush"));
                    }
                }
            }
        });
}
