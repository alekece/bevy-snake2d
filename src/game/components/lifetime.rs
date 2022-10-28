use bevy::prelude::*;

#[derive(Debug, Clone, Default, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct Lifetime(Timer);

impl Lifetime {
    pub fn from_seconds(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, false))
    }
}
