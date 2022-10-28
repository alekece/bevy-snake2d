use std::collections::hash_map::Entry;
use std::collections::HashMap;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use itertools::Itertools;
use iyes_loopless::prelude::*;
// use serde::{Deserialize, Serialize};

use super::components::{Position, RigidBody};
use super::{GameStage, GameSystem};

pub const TILE_SIZE: f32 = 128.;

pub enum TileMapPosition {
    Centered,
    Offset(Vec2),
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "kebab-case")]
// pub enum TileEntity {
//     Bush,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TileBuilder {
//     pub x: u32,
//     pub y: u32,
//     pub entity: TileEntity,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TileMapBuilder {
//     pub width: usize,
//     pub height: usize,
//     pub tiles: Vec<TileBuilder>
// }

pub struct TileMapOptions {
    pub tile_size: f32,
    pub position: TileMapPosition,
}

pub struct Tile {
    entity: Option<Entity>,
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
        self.entity.is_none()
    }
}

pub struct TileMap {
    tiles: Vec<Tile>,
    size: UVec2,
    entities: HashMap<Entity, UVec2>,
}

impl TileMap {
    pub fn empty(width: u32, height: u32) -> Self {
        Self {
            tiles: (0..height)
                .into_iter()
                .cartesian_product(0..width)
                .map(|(y, x)| Tile {
                    entity: None,
                    position: UVec2::new(x, y),
                })
                .collect(),
            size: UVec2::new(width, height),
            entities: HashMap::default(),
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
        app.add_system(
            clamp_position
                .run_if_resource_exists::<TileMap>()
                .label(GameSystem::CheckPosition)
                .after(GameSystem::Movement),
        )
        .add_system(
            update_tiles
                .run_if_resource_exists::<TileMap>()
                .after(GameSystem::CheckPosition),
        )
        .add_system_set_to_stage(
            GameStage::Transform,
            ConditionSet::new()
                .run_if_resource_exists::<TileMap>()
                .run_if_resource_exists::<TileMapOptions>()
                .with_system(position_to_world)
                .with_system(size_to_world)
                .into(),
        )
        .add_system_to_stage(GameStage::Cleanup, clean_tiles.run_if_resource_exists::<TileMap>());
    }
}

fn clean_tiles(entities: RemovedComponents<RigidBody>, mut tile_map: ResMut<TileMap>) {
    entities.iter().for_each(|entity| {
        if let Some(mut tile) = tile_map
            .entities
            .remove(&entity)
            .and_then(|position| tile_map.tile_mut_at(position.x, position.y))
        {
            tile.entity = None;
        }
    });
}

fn update_tiles(
    query: Query<(Entity, &Position), (With<RigidBody>, Changed<Position>)>,
    mut tile_map: ResMut<TileMap>,
) {
    query.for_each(|(entity, position)| {
        let position = position.0.xy().as_uvec2();

        let previous_position = match tile_map.entities.entry(entity) {
            Entry::Occupied(mut entry) => {
                let previous_position = *entry.get();

                *entry.get_mut() = position;

                Some(previous_position)
            }
            // the entity is just added
            Entry::Vacant(entry) => {
                entry.insert(position);

                None
            }
        };

        // remove the entity from its previous tile
        if let Some(tile) = previous_position.and_then(|position| tile_map.tile_mut_at(position.x, position.y)) {
            tile.entity = None;
        }

        // then add the entity to its current tile
        if let Some(tile) = tile_map.tile_mut_at(position.x, position.y) {
            tile.entity = Some(entity);
        }
    });
}

pub fn clamp_position(mut query: Query<&mut Position, Changed<Position>>, tile_map: Res<TileMap>) {
    query.for_each_mut(|mut position| {
        let width = tile_map.width() as i32;
        let height = tile_map.height() as i32;

        if position.0.x >= width {
            position.0.x -= width;
        } else if position.0.x < 0 {
            position.0.x += width;
        }
        if position.0.y >= height {
            position.0.y -= height;
        } else if position.0.y < 0 {
            position.0.y += height;
        }
    });
}

pub fn position_to_world(
    mut query: Query<(&mut Transform, &Position)>,
    tile_map: Res<TileMap>,
    tile_map_options: Res<TileMapOptions>,
) {
    query.for_each_mut(|(mut transform, position)| {
        let offset = match tile_map_options.position {
            TileMapPosition::Centered => {
                let half_tile_size = tile_map_options.tile_size / 2.0;

                Vec2::new(
                    -(tile_map.width() as f32 * tile_map_options.tile_size / 2.0) + half_tile_size,
                    -(tile_map.height() as f32 * tile_map_options.tile_size / 2.0) + half_tile_size,
                )
            }
            TileMapPosition::Offset(offset) => offset,
        };

        transform.translation = Vec3::new(
            position.0.x as f32 * tile_map_options.tile_size + offset.x,
            position.0.y as f32 * tile_map_options.tile_size + offset.y,
            position.0.z as f32,
        );
    });
}

pub fn size_to_world(mut query: Query<&mut Sprite>, tile_map_options: Res<TileMapOptions>) {
    query.for_each_mut(|mut sprite| {
        sprite.custom_size = Some(Vec2::splat(tile_map_options.tile_size));
    });
}
