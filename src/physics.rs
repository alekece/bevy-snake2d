use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component)]
pub struct RigidBody;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct Position(pub IVec3);

#[derive(Debug, Default, Copy, Clone, PartialEq, Component, Reflect)]
#[reflect(Component)]
pub struct Size(pub Vec2);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect_value(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    #[default]
    Left,
}

impl From<Direction> for IVec3 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new(0, 1, 0),
            Direction::Down => Self::new(0, -1, 0),
            Direction::Right => Self::new(1, 0, 0),
            Direction::Left => Self::new(-1, 0, 0),
        }
    }
}

impl From<Direction> for Quat {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::from_rotation_z(-90. * PI / 180.),
            Direction::Down => Self::from_rotation_z(90. * PI / 180.),
            Direction::Left => Self::from_rotation_z(0. * PI / 180.),
            Direction::Right => Self::from_rotation_z(180. * PI / 180.),
        }
    }
}

impl Direction {
    pub const fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub struct PreviousDirection(pub Direction);
