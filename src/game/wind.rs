use std::ops::Range;
use std::time::Duration;

use bevy::prelude::*;
use bevy_tweening::lens::TransformScaleLens;
use bevy_tweening::{Animator, EaseFunction, Sequence, Tween, TweeningType};
use derive_more::{Deref, DerefMut};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::run_criterias;

use super::components::{Direction, NumberGenerator, Position};
use super::GameSystem;

#[derive(Component)]
pub struct Windable;

#[derive(Component)]
pub struct Wind;

#[derive(Debug, Deref, DerefMut)]
pub struct WindTimer {
    #[deref]
    #[deref_mut]
    inner: Timer,
    generator: NumberGenerator<f32>,
}

impl WindTimer {
    pub fn new(range: Range<f32>) -> Self {
        let mut generator = NumberGenerator::from_range(range);

        Self {
            inner: Timer::from_seconds(generator.generate(), false),
            generator,
        }
    }
}

pub struct WindPlugin;

impl Plugin for WindPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_wind.run_if(run_criterias::timer_finished::<WindTimer>))
            .add_system(move_wind.label(GameSystem::Movement))
            .add_system(apply_wind.after(GameSystem::Movement));
    }
}

fn spawn_wind(mut commands: Commands, mut timer: ResMut<WindTimer>) {
    commands
        .spawn()
        .insert_bundle((Position(IVec3::new(0, 0, 1)), Direction::Up, Wind, Name::new("Wind")));

    timer.inner = Timer::from_seconds(timer.generator.generate(), false);
}

fn move_wind(mut wind_query: Query<(&mut Position, &Direction), With<Wind>>) {
    wind_query.for_each_mut(|(mut position, direction)| {
        *position += direction.to_ivec3();
    });
}

fn apply_wind(
    mut commands: Commands,
    windable_query: Query<(Entity, &Position, &Transform), (With<Windable>, Without<Wind>)>,
    wind_query: Query<(&Position, &Direction), With<Wind>>,
) {
    wind_query.for_each(|(position, direction)| {
        windable_query.for_each(|(entity, windable_position, windable_transform)| {
            match matches!(direction, Direction::Up | Direction::Down) {
                true if windable_position.y == position.y => {
                    println!("youhou");
                    commands.entity(entity)
                            .remove::<Windable>()
                            .insert(Animator::new(Sequence::new([
                        Tween::new(
                            EaseFunction::SineIn,
                            TweeningType::Once,
                            Duration::from_millis(2500),
                            TransformScaleLens {
                                start: windable_transform.scale,
                                end: Vec3::new(
                                    windable_transform.scale.x,
                                    windable_transform.scale.y * 1.5,
                                    windable_transform.scale.z,
                                ),
                            },
                        ),
                        Tween::new(
                            EaseFunction::SineOut,
                            TweeningType::Once,
                            Duration::from_millis(2500),
                            TransformScaleLens {
                                start: Vec3::new(
                                    windable_transform.scale.x,
                                    windable_transform.scale.y * 1.5,
                                    windable_transform.scale.z,
                                ),
                                end: windable_transform.scale,
                            },
                        ),
                    ])));
                }
                false if windable_position.x == position.x => {}
                _ => (),
            };
        });
    });
}
