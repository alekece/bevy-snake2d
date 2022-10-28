use std::ops::DerefMut;

use bevy::prelude::*;

pub fn update_timer<T: Component + DerefMut<Target = Timer>>(mut timer_query: Query<&mut T>, time: Res<Time>) {
    timer_query.for_each_mut(|mut timer| {
        timer.deref_mut().tick(time.delta());
    });
}
