use bevy::prelude::*;

pub fn despawn_all(mut commands: Commands, query: Query<Entity>) {
    query.for_each(|e| commands.entity(e).despawn_recursive());
}

pub fn despawn_with<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    query.for_each(|e| commands.entity(e).despawn_recursive());
}

pub fn despawn_without<T: Component>(mut commands: Commands, query: Query<Entity, Without<T>>) {
    query.for_each(|e| commands.entity(e).despawn_recursive());
}
