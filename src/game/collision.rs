use std::marker::PhantomData;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use crate::physics::{Position, RigidBody};

use super::GameStep;

#[derive(Component)]
pub struct Collide;

pub struct CollisionEvent(Entity, Entity);
pub struct DiscreteCollisionEvent<T, U>(pub Entity, pub Entity, PhantomData<(T, U)>);

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>()
            .add_system(detect_collision.label(GameStep::Collision).after(GameStep::Move));
    }
}

pub struct DiscreteCollisionPlugin<T, U>(PhantomData<(T, U)>);

impl<T, U> Default for DiscreteCollisionPlugin<T, U> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T, U> Plugin for DiscreteCollisionPlugin<T, U>
where
    T: Component,
    U: Component,
{
    fn build(&self, app: &mut App) {
        app.add_event::<DiscreteCollisionEvent<T, U>>().add_system(
            detect_discrete_collision::<T, U>
                .label(GameStep::Collision)
                .after(GameStep::Move),
        );
    }
}

pub fn detect_collision(
    mut collision_writer: EventWriter<CollisionEvent>,
    query: Query<(Entity, &Position), With<RigidBody>>,
) {
    query.iter_combinations().for_each(|[a, b]| {
        if a.1 .0.xy() == b.1 .0.xy() {
            collision_writer.send(CollisionEvent(a.0, b.0))
        }
    });
}

pub fn detect_discrete_collision<T: Component, U: Component>(
    mut collision_reader: EventReader<CollisionEvent>,
    mut discrete_collision_writer: EventWriter<DiscreteCollisionEvent<T, U>>,
    mut queries: ParamSet<(Query<(), With<T>>, Query<(), With<U>>)>,
) {
    for event in collision_reader.iter() {
        if queries.p0().get(event.0).is_ok() && queries.p1().get(event.1).is_ok() {
            discrete_collision_writer.send(DiscreteCollisionEvent(event.0, event.1, PhantomData));
        } else if queries.p0().get(event.1).is_ok() && queries.p1().get(event.0).is_ok() {
            discrete_collision_writer.send(DiscreteCollisionEvent(event.1, event.0, PhantomData));
        }
    }
}
