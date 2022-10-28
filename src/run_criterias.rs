use std::ops::DerefMut;

use bevy::ecs::system::Resource;
use bevy::prelude::*;

pub fn timer_finished<T: Resource + DerefMut<Target = Timer>>(time: Res<Time>, timer: Option<ResMut<T>>) -> bool {
    timer.map_or(false, |mut timer| {
        timer.tick(time.delta());

        timer.finished()
    })
}
