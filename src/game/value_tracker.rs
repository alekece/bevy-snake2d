use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::marker::PhantomData;

use bevy::prelude::*;
use derive_more::Deref;

use super::GameStage;

#[derive(Debug, Clone, Component, Deref)]
pub struct PreviousValue<T: Component + Clone>(pub T);

struct ValueTracker<T>(HashMap<Entity, T>);

impl<T> Default for ValueTracker<T> {
    fn default() -> Self {
        Self(HashMap::default())
    }
}

pub struct ValueTrackerPlugin<T>(PhantomData<T>);

impl<T> Default for ValueTrackerPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Component + Clone> Plugin for ValueTrackerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<ValueTracker<T>>()
            .add_system_to_stage(GameStage::Track, track_entity::<T>);
    }
}

fn track_entity<T: Component + Clone>(
    mut commands: Commands,
    query: Query<(Entity, &T), Changed<T>>,
    mut value_tracker: ResMut<ValueTracker<T>>,
) {
    query.for_each(|(entity, component)| match value_tracker.0.entry(entity) {
        Entry::Occupied(mut entry) => {
            let previous_component = entry.insert(component.clone());

            commands.entity(entity).insert(PreviousValue(previous_component));
        }
        Entry::Vacant(entry) => {
            entry.insert(component.clone());
        }
    });
}
