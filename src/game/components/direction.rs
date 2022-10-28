use std::f32::consts::PI;

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use derive_more::Display;

#[derive(Debug, Display, Default, Copy, Clone, PartialEq, Eq, Component, Reflect)]
#[reflect_value()]
pub enum Direction {
    #[default]
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    #[inline]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }

    #[inline]
    pub const fn clockwise(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    #[inline]
    pub const fn counter_clockwise(self) -> Self {
        self.clockwise().opposite()
    }

    #[inline]
    pub fn to_quat(self) -> Quat {
        match self {
            Self::Up => Quat::from_rotation_z(-90. * PI / 180.),
            Self::Down => Quat::from_rotation_z(90. * PI / 180.),
            Self::Left => Quat::from_rotation_z(0. * PI / 180.),
            Self::Right => Quat::from_rotation_z(180. * PI / 180.),
        }
    }

    #[inline]
    pub fn to_ivec3(self) -> IVec3 {
        match self {
            Self::Up => IVec3::new(0, 1, 0),
            Self::Down => IVec3::new(0, -1, 0),
            Self::Right => IVec3::new(1, 0, 0),
            Self::Left => IVec3::new(-1, 0, 0),
        }
    }

    #[inline]
    pub fn to_ivec2(self) -> IVec2 {
        self.to_ivec3().xy()
    }

    #[inline]
    pub fn to_vec3(self) -> Vec3 {
        self.to_ivec3().as_vec3()
    }

    #[inline]
    pub fn to_vec2(self) -> Vec2 {
        self.to_ivec2().as_vec2()
    }
}

impl From<Direction> for IVec3 {
    fn from(direction: Direction) -> Self {
        direction.to_ivec3()
    }
}

impl From<Direction> for IVec2 {
    fn from(direction: Direction) -> Self {
        direction.to_ivec2()
    }
}

impl From<Direction> for Vec3 {
    fn from(direction: Direction) -> Self {
        direction.to_vec3()
    }
}

impl From<Direction> for Vec2 {
    fn from(direction: Direction) -> Self {
        direction.to_vec2()
    }
}

impl From<Direction> for Quat {
    fn from(direction: Direction) -> Self {
        direction.to_quat()
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;

    #[test]
    fn it_constructs_opposite_direction() {
        let tests = [(Direction::Up, Direction::Down), (Direction::Left, Direction::Right)];

        for (direction, opposite_direction) in tests {
            assert_eq!(
                direction.opposite(),
                opposite_direction,
                "{} is the opposite direction of {}",
                opposite_direction,
                direction,
            );

            assert_eq!(
                opposite_direction.opposite(),
                direction,
                "{} is the opposite direction of {}",
                direction,
                opposite_direction
            );
        }
    }
}
