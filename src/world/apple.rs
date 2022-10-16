use std::f32::consts::PI;

use bevy::prelude::*;
use iyes_loopless::prelude::*;
use rand::Rng;

use super::tile_map::{Tile, TileMap};
use crate::physics::Position;
use crate::Assets;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Apple;

pub struct AppleSpawner {
    pub timer: Timer,
    pub max_apples: usize,
}

pub struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_apple);

        #[cfg(feature = "debug")]
        {
            app.register_type::<Apple>();
        }
    }
}

fn spawn_apple(
    mut commands: Commands,
    query: Query<&Apple>,
    time: Res<Time>,
    mut apple_spawner: ResMut<AppleSpawner>,
    tile_map: Res<TileMap>,
    assets: Res<Assets>,
) {
    apple_spawner.timer.tick(time.delta());

    if apple_spawner.timer.just_finished() {
        let apple_count = query.iter().count();

        if apple_count < apple_spawner.max_apples {
            let mut rng = rand::thread_rng();

            let tiles = tile_map.tiles().filter(|tile| tile.is_empty()).collect::<Vec<_>>();

            let position = tiles[rng.gen_range(0..tiles.len())].position();
            let rotation = Quat::from_rotation_z(rng.gen_range(0.0..360.0) * PI / 180.0);

            commands
                .spawn_bundle(SpriteBundle {
                    texture: assets.apple_texture.clone(),
                    transform: Transform::from_rotation(rotation),
                    ..Default::default()
                })
                .insert(Apple)
                .insert(Position(IVec3::new(position.x as i32, position.y as i32, 1)))
                .insert(Name::new("Apple"));
        }
    }
}
