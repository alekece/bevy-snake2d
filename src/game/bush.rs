use std::{f32::consts::PI, time::Duration};

use bevy::{prelude::*, utils::tracing::instrument::WithSubscriber};
use bevy_tweening::{
    lens::{TransformRotateZLens, TransformScaleLens},
    Animator, EaseFunction, Tracks, Tween, TweeningType,
};

use super::{components::{Position, RigidBody}, wind::Windable};
use crate::assets::TextureAssets;

#[derive(Debug, Component)]
pub struct Bush;

#[derive(Default)]
pub struct BushBuilder {
    position: Option<IVec3>,
    angle: Option<f32>,
    is_animated: bool,
}

impl BushBuilder {
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
        let translation = self.position.unwrap_or_default().as_vec3();

        let mut commands = commands.spawn_bundle(SpatialBundle {
            transform: Transform::from_rotation(Quat::from_rotation_z(radian)).with_scale(Vec3::new(1.15, 1.15, 1.0)),
            ..Default::default()
        });

        commands
            .insert_bundle((Bush, RigidBody, Windable, Name::new("Bush")))
            .with_children(|parent| {
                let mut lower_bush = parent.spawn_bundle(SpriteBundle {
                    texture: textures.bush_lower.clone(),
                    ..Default::default()
                });

                if self.is_animated {
                    lower_bush.insert(Animator::new(Tween::new(
                        EaseFunction::CubicInOut,
                        TweeningType::PingPong,
                        Duration::from_millis(5500),
                        TransformRotateZLens {
                            start: radian,
                            end: radian + 0.2,
                        },
                    )));
                }

                let mut upper_bush = parent.spawn_bundle(SpriteBundle {
                    texture: textures.bush_upper.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, translation.z + 0.1),
                    ..Default::default()
                });

                if self.is_animated {
                    upper_bush.insert(Animator::new(Tracks::new([
                        Tween::new(
                            EaseFunction::CubicInOut,
                            TweeningType::PingPong,
                            Duration::from_millis(5500),
                            TransformRotateZLens {
                                start: radian,
                                end: radian + 0.1,
                            },
                        ),
                        Tween::new(
                            EaseFunction::CubicInOut,
                            TweeningType::PingPong,
                            Duration::from_millis(200),
                            TransformScaleLens {
                                start: Vec3::new(1.0, 1.0, 1.0),
                                end: Vec3::new(1.2, 1.0, 1.0),
                            },
                        ),
                        Tween::new(
                            EaseFunction::CubicInOut,
                            TweeningType::PingPong,
                            Duration::from_millis(200),
                            TransformScaleLens {
                                start: Vec3::new(1.0, 1.0, 1.0),
                                end: Vec3::new(1.0, 1.2, 1.0),
                            },
                        ),

                    ])));
                }
            });

        if let Some(position) = self.position {
            commands.insert(Position(position));
        }

        // if self.is_animated {
        //     commands.insert(Animator::new(Tween::new(
        //         EaseFunction::SineInOut,
        //         TweeningType::PingPong,
        //         Duration::from_millis(500),
        //         TransformScaleLens {
        //             start: Vec3::splat(1.0),
        //             end: Vec3::splat(1.05),
        //         },
        //     )));
        // }

        commands.id()
    }
}

pub struct BushPlugin;

impl Plugin for BushPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate_bush);
    }
}

fn animate_bush() {}
