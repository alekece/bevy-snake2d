use bevy::ecs::query::WorldQuery;

use crate::game::components::{Direction, Position};
use crate::game::value_tracker::PreviousValue;

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct SpatialQuery {
    pub position: &'static mut Position,
    pub direction: DirectionQuery,
}

#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct DirectionQuery {
    pub current: &'static mut Direction,
    pub previous: Option<&'static PreviousValue<Direction>>,
}
