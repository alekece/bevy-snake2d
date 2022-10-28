use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::prelude::*;
use derive_more::{Deref, DerefMut};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Component, Reflect, Deref, DerefMut)]
#[reflect(Component)]
pub struct Position(pub IVec3);

impl Add<IVec3> for Position {
    type Output = Self;

    fn add(self, rhs: IVec3) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<IVec3> for Position {
    fn add_assign(&mut self, rhs: IVec3) {
        self.0 += rhs;
    }
}

impl Sub<IVec3> for Position {
    type Output = Self;

    fn sub(self, rhs: IVec3) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl SubAssign<IVec3> for Position {
    fn sub_assign(&mut self, rhs: IVec3) {
        self.0 -= rhs;
    }
}
