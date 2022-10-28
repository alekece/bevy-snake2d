use bevy::prelude::*;
use std::ops::DerefMut;

pub struct TimerPlugin<T>(PhantomData);

impl<T> Default for TimerPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl Plugin for TimerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(update_timer::<T>.label(GameSystem::UpdateTimer))
    }
}

pub fn update_timer<T: Component + DerefMut<Target = Timer>>(mut timer_query: Query<&mut T>, time: Res<Time>) {
    timer_query.for_each_mut(|mut timer| {
        timer.deref_mut().tick(time.delta());
    });
}
