use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::lens::{TransformRotateZLens, TransformRotationLens, TransformScaleLens};
use bevy_tweening::{Animator, EaseFunction, Tween, TweeningType};
use iyes_loopless::prelude::*;
use rand::Rng;

use super::components::{Lifetime, Position, RigidBody};
use super::tile_map::TileMap;
use super::GameSystem;
use crate::assets::TextureAssets;
use crate::systems;

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct Apple;

#[derive(Default)]
pub struct AppleBuilder {
    position: Option<IVec3>,
    angle: Option<f32>,
    is_animated: bool,
}

impl AppleBuilder {
    pub fn with_position(mut self, position: IVec3) -> Self {
        self.position = Some(position);

        self
    }

    pub fn with_angle(mut self, angle: f32) -> Self {
        self.angle = Some(angle);

        self
    }

    pub fn animate(mut self) -> Self {
        self.is_animated = true;

        self
    }

    pub fn spawn(self, commands: &mut Commands, textures: &TextureAssets) -> Entity {
        let radian = self.angle.map_or(0.0, f32::to_radians);
        let position = Position(self.position.unwrap_or_default());

        let mut commands = commands.spawn_bundle(SpriteBundle {
            texture: textures.apple.clone(),
            transform: Transform::from_rotation(Quat::from_rotation_z(radian)),
            ..Default::default()
        });

        commands
            .insert_bundle((Apple, RigidBody, position, Name::new("Apple")))
            .with_children(|parent| {
                let mut apple_leaf = parent.spawn_bundle(SpriteBundle {
                    texture: textures.apple_leaf.clone(),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, position.0.z as f32 + 0.1)),
                    ..Default::default()
                });

                if self.is_animated {
                    apple_leaf.insert(Animator::new(Tween::new(
                        EaseFunction::ElasticOut,
                        TweeningType::Once,
                        Duration::from_millis(2500),
                        TransformRotateZLens {
                            start: radian,
                            end: radian + 1.5,
                        },
                    )));
                }
            });

        if self.is_animated {
            commands.insert(Animator::new(Tween::new(
                EaseFunction::BounceOut,
                TweeningType::Once,
                Duration::from_millis(1000),
                TransformScaleLens {
                    start: Vec3::new(2.0, 2.0, 1.0),
                    end: Vec3::new(1.0, 1.0, 1.0),
                },
            )));
        }
        // if let Some(lifetime) = apple_spawner.apple_lifetime.clone() {
        //     commands.entity(entity).insert(lifetime);
        // }

        commands.id()
    }
}

#[derive(Debug, Clone)]
pub struct AppleSpawner {
    pub timer: Timer,
    pub max_apples: usize,
    pub apple_lifetime: Option<Lifetime>,
}

pub struct ApplePlugin;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn_apple
                .run_if_resource_exists::<AppleSpawner>()
                .run_if_resource_exists::<TileMap>()
                .before(GameSystem::Movement),
        )
        .add_system(systems::update_timer::<Lifetime>)
        .add_system(explode_apple);
    }
}

pub fn spawn_apple(
    mut commands: Commands,
    query: Query<&Apple>,
    time: Res<Time>,
    mut apple_spawner: ResMut<AppleSpawner>,
    tile_map: Res<TileMap>,
    textures: Res<TextureAssets>,
) {
    apple_spawner.timer.tick(time.delta());

    if apple_spawner.timer.just_finished() {
        let apple_count = query.iter().count();

        if apple_count < apple_spawner.max_apples {
            let mut rng = rand::thread_rng();

            let tiles = tile_map.tiles().filter(|tile| tile.is_empty()).collect::<Vec<_>>();

            let position = tiles[rng.gen_range(0..tiles.len())].position();
            let angle = rng.gen_range(0.0..360.0);

            AppleBuilder::default()
                .with_position(IVec3::new(position.x as i32, position.y as i32, 1))
                .with_angle(angle)
                .animate()
                .spawn(&mut commands, &*textures);
        }
    }
}

pub fn explode_apple(mut commands: Commands, mut query: Query<(Entity, &mut Lifetime), With<Apple>>) {
    query.for_each_mut(|(entity, lifetime)| {
        if lifetime.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    });
}
