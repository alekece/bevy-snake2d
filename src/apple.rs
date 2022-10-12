use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use rand::Rng;

use crate::board::Board;
use crate::snake::Position;
use crate::{GameAssets, TILE_SIZE};

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Apple;

pub struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.))
                .with_system(spawn_apple),
        );

        #[cfg(feature = "debug")]
        {
            app.register_type::<Apple>();
        }
    }
}

fn spawn_apple(mut commands: Commands, board: Res<Board>, game_assets: Res<GameAssets>) {
    let mut rng = rand::thread_rng();
    let x_bound = board.size.x / 2;
    let y_bound = board.size.y / 2;
    let rotation = Quat::from_rotation_z(rng.gen_range(0.0..360.0)*PI/180.0);


    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..Default::default()
            },
            texture: game_assets.apple_texture.clone(),
            transform: Transform::from_rotation(rotation),
            ..Default::default()
        })
        .insert(Apple)
        .insert(Position(IVec3::new(
            rng.gen_range(-x_bound..x_bound),
            rng.gen_range(-y_bound..y_bound),
            1,
        )))
        .insert(Name::new("Apple"));
}
