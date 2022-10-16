// use std::f32::consts::PI;
// use std::marker::PhantomData;

// use bevy::prelude::*;
// use rand::Rng;

// use crate::Assets;
// use crate::components::{Position, Size, RigidBody};

// use super::apple::Apple;
// use super::snake::{SnakeFragment, SnakeHead};

// const BOARD_TILE_COLOR: Color = Color::rgb(0.5, 0.9, 0.19);
// const BOARD_TILE_SIZE: Vec2 = Vec2::splat(0.99);

// pub struct CollisionEvent<T, U> {
//     pub collider: Entity,
//     pub collided: Entity,
//     _marker: PhantomData<(T, U)>,
// }

// impl<T, U> CollisionEvent<T, U> {
//     pub fn new(collider: Entity, collided: Entity) -> Self {
//         Self {
//             collided,
//             collider,
//             _marker: PhantomData,
//         }
//     }
// }

use std::collections::HashSet;

use bevy::prelude::*;
use itertools::Itertools;
use iyes_loopless::prelude::*;

use crate::physics::Position;

pub struct Tile {
    entities: HashSet<Entity>,
    position: UVec2,
}

impl Tile {
    pub const fn x(&self) -> u32 {
        self.position.x
    }

    pub const fn y(&self) -> u32 {
        self.position.y
    }

    pub const fn position(&self) -> UVec2 {
        self.position
    }

    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    pub fn add_entity(&mut self, e: Entity) -> bool {
        self.entities.insert(e)
    }

    pub fn remove_entity(&mut self, e: Entity) -> bool {
        self.entities.remove(&e)
    }
}

pub struct TileMap {
    tiles: Vec<Tile>,
    size: UVec2,
}

impl TileMap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            tiles: (0..height)
                .into_iter()
                .cartesian_product(0..width)
                .map(|(x, y)| {
                    Tile {
                        entities: default(),
                        position: UVec2::new(x, y),
                    }
                })
                .collect(),
            size: UVec2::new(width, height),
        }
    }

    pub const fn width(&self) -> u32 {
        self.size.x
    }

    pub const fn height(&self) -> u32 {
        self.size.y
    }

    fn position_to_index(&self, x: u32, y: u32) -> usize {
        (y * self.width() + x) as usize
    }

    pub fn tile_at(&self, x: u32, y: u32) -> Option<&Tile> {
        let i = self.position_to_index(x, y);

        self.tiles.get(i)
    }

    pub fn tile_mut_at(&mut self, x: u32, y: u32) -> Option<&mut Tile> {
        let i = self.position_to_index(x, y);

        self.tiles.get_mut(i)
    }

    pub fn tiles(&self) -> impl Iterator<Item = &Tile> {
        self.tiles.iter()
    }

    pub fn tiles_mut(&mut self) -> impl Iterator<Item = &mut Tile> {
        self.tiles.iter_mut()
    }
}

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                // .run_if_resource_added::<TileMap>()
                .with_system(add_entities)
                .with_system(clamp_position)
                .into(),
        );
    }
}

fn add_entities(query: Query<(Entity, &Position), Added<Position>>, mut tile_map: ResMut<TileMap>) {
    query.for_each(|(e, position)| {
        println!("add entities to tilemap");

        if let Some(tile) = tile_map.tile_mut_at(position.0.x as u32, position.0.y as u32) {
            tile.add_entity(e);
        }
    })
}

fn clamp_position(mut query: Query<&mut Position, Changed<Position>>, tile_map: Res<TileMap>) {
    query.for_each_mut(|mut position| {
        let x_bound = (tile_map.width() / 2) as i32;
        let y_bound = (tile_map.height() / 2) as i32;

        if position.0.x >= x_bound {
            position.0.x -= tile_map.width() as i32;
        } else if position.0.x < -x_bound {
            position.0.x += tile_map.width() as i32;
        }
        if position.0.y >= y_bound {
            position.0.y -= tile_map.height() as i32;
        } else if position.0.y < -y_bound {
            position.0.y += tile_map.height() as i32;
        }
    });
}
// fn remove_entities(event_reader: EventReader<TileMapEvent>, query: Query<&Position>, mut tile_map: ResMut<TileMap>) {
//     for event in event_reader.iter {
//         if let Some(position) = query.get() {

//         }
//     }
// }
// pub struct BoardPlugin;

// impl Plugin for BoardPlugin {
//     fn build(&self, app: &mut App) {
//         app.init_resource::<Board>()
//             .add_event::<CollisionEvent<SnakeHead, Apple>>()
//             .add_event::<CollisionEvent<SnakeHead, RigidBody>>()
//             .add_event::<CollisionEvent<SnakeHead, SnakeFragment>>()
//             .add_system(game_over::<SnakeFragment>)
//             .add_system(game_over::<RigidBody>)
//             .add_system(clamp_position.after("move"))
//             .add_system(detect_collision::<SnakeHead, Apple>.after("move"))
//             .add_system(detect_collision::<SnakeHead, SnakeFragment>.after("move"))
//             .add_system(detect_collision::<SnakeHead, RigidBody>.after("move"));
//     }
// }

// pub fn detect_collision<T, U>(
//     mut collision_writer: EventWriter<CollisionEvent<T, U>>,
//     moved_entities: Query<(Entity, &Position), (With<T>, Changed<Position>)>,
//     entities: Query<(Entity, &Position), (Without<T>, With<U>)>,
// ) where
//     T: Component,
//     U: Component,
// {
//     moved_entities.for_each(|(moved_entity, moved_position)| {
//         entities.for_each(|(entity, position)| {
//             if moved_entity != entity && moved_position == position {
//                 collision_writer.send(CollisionEvent::new(moved_entity, entity));
//             }
//         });
//     });
// }

// fn clamp_position(board: Res<Board>, mut positions: Query<&mut Position>) {
//     for mut position in positions.iter_mut() {
//         let x_bound = board.size.x / 2;
//         let y_bound = board.size.y / 2;

//         if position.0.x >= x_bound {
//             position.0.x -= board.size.x;
//         } else if position.0.x < -x_bound {
//             position.0.x += board.size.x;
//         }
//         if position.0.y >= y_bound {
//             position.0.y -= board.size.y;
//         } else if position.0.y < -y_bound {
//             position.0.y += board.size.y;
//         }
//     }
// }

// fn spawn_board(mut commands: Commands, game_assets: Res<GameAssets>, board: Res<Board>) {
//     let x_bound = board.size.x / 2;
//     let y_bound = board.size.y / 2;
//     let mut rng = rand::thread_rng();

//     commands
//         .spawn()
//         .insert_bundle(TransformBundle::default())
//         .insert_bundle(VisibilityBundle::default())
//         .insert(Name::new("Board"))
//         .with_children(|parent| {
//             for y in -y_bound..y_bound {
//                 for x in -x_bound..x_bound {
//                     parent
//                         .spawn_bundle(SpriteBundle {
//                             sprite: Sprite {
//                                 color: BOARD_TILE_COLOR,
//                                 ..Default::default()
//                             },
//                             ..Default::default()
//                         })
//                         .insert(Size(BOARD_TILE_SIZE))
//                         .insert(Position(IVec3::new(x, y, 0)))
//                         .insert(Name::new(format!("Tile ({x}, {y})")));

//                     // if x == -x_bound || x == x_bound - 1 || y == -y_bound || y == y_bound - 1 {
//                     //     parent
//                     //         .spawn_bundle(SpriteBundle {
//                     //             texture: game_assets.box_texture.clone(),
//                     //             ..Default::default()
//                     //         })
//                     //         .insert(Position(IVec3::new(x, y, 1)))
//                     //         .insert(RigidBody)
//                     //         .insert(Name::new("Box"));
//                     // }
//                     if rng.gen_bool(1. / 10.) {
//                         parent
//                             .spawn_bundle(SpriteBundle {
//                                 texture: game_assets.bush_texture.clone(),
//                                 transform: Transform::from_rotation(Quat::from_rotation_z(
//                                     rng.gen_range(0.0..180.) * PI / 180.,
//                                 )).with_scale(Vec3::new(1., 1., 0.5)),
//                                 //     .with_rotation(Quat::from_rotation_z(rng.gen_range(0.0..180.) * PI / 180.)),
//                                 ..Default::default()
//                             })
//                             // .insert(RigidBody)
//                             .insert(Position(IVec3::new(x, y, 1)))
//                             .insert(Name::new("Bush"));
//                     }
//                 }
//             }
//         });
// }
