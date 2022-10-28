use std::mem;
use std::num::NonZeroUsize;

use bevy::prelude::*;
use derive_more::{Deref, DerefMut};
use leafwing_input_manager::prelude::*;

use crate::assets::TextureAssets;
use crate::systems;

use super::apple::Apple;
use super::components::{Direction, NumberGenerator, Position, RigidBody};
use super::queries::spatial::DirectionQuery;
use super::queries::SpatialQuery;
use super::{GameStage, GameSystem};

#[derive(Debug, Component, Reflect)]
pub struct Snake {
    #[reflect(ignore)]
    pub fragments: Vec<Entity>,
    pub next_direction: Option<Direction>,
    pub last_known_tail: Option<(Position, Direction)>,
}

#[derive(Debug, Component, Deref, DerefMut)]
pub struct MoveTimer(Timer);

#[derive(Debug)]
pub enum SnakeEvent {
    HeadCollide(Entity),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Actionlike)]
pub enum SnakeAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

#[derive(Debug, Copy, Clone, Component, Reflect)]
#[reflect_value()]
pub enum SnakeFragment {
    Head,
    Body(u16),
    Tail,
}

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SnakeEvent>()
            .add_plugin(InputManagerPlugin::<SnakeAction>::default())
            .add_system_set(
                SystemSet::new()
                    .before(GameSystem::Movement)
                    .with_system(turn_snake_head)
                    .with_system(systems::update_timer::<MoveTimer>),
            )
            .add_system(
                move_snake
                    .label(GameSystem::Movement)
                    .before(GameSystem::CollisionDetection),
            )
            .add_system_set(
                SystemSet::new()
                    .after(GameSystem::CheckPosition)
                    .label(GameSystem::CollisionDetection)
                    .with_system(grow_snake), // .with_system(check_snake_head_collision),
            )
            .add_system_set_to_stage(
                GameStage::Transform,
                SystemSet::new()
                    .with_system(update_snake_sprite)
                    .with_system(update_snake_transform),
            );
    }
}

fn turn_snake_head(
    mut snake_query: Query<(&ActionState<SnakeAction>, &mut Snake)>,
    mut direction_query: Query<&Direction>,
) {
    snake_query.for_each_mut(|(action, mut snake)| {
        let next_direction = if action.pressed(SnakeAction::MoveUp) {
            Direction::Up
        } else if action.pressed(SnakeAction::MoveLeft) {
            Direction::Left
        } else if action.pressed(SnakeAction::MoveRight) {
            Direction::Right
        } else if action.pressed(SnakeAction::MoveDown) {
            Direction::Down
        } else {
            return;
        };

        let direction = direction_query.get_mut(snake.fragments[0]).unwrap();

        if next_direction != direction.opposite() {
            snake.next_direction = Some(next_direction);
        }
    });
}

fn move_snake(mut snake_query: Query<(&mut Snake, &MoveTimer)>, mut spatial_query: Query<SpatialQuery>) {
    snake_query.for_each_mut(|(mut snake, timer)| {
        if timer.just_finished() {
            // save the tail position and direction to ease snake growth later on
            let snake_tail = spatial_query.get(*snake.fragments.last().unwrap()).unwrap();
            snake.last_known_tail = Some((*snake_tail.position, *snake_tail.direction.current));

            let mut snake_head = spatial_query.get_mut(snake.fragments[0]).unwrap();

            *snake_head.direction.current = snake.next_direction.take().unwrap_or(*snake_head.direction.current);

            let (mut position, mut direction) = (
                *snake_head.position + snake_head.direction.current.to_ivec3(),
                *snake_head.direction.current,
            );

            let mut iter = spatial_query.iter_many_mut(&snake.fragments[0..]);

            while let Some(mut snake_fragment) = iter.fetch_next() {
                mem::swap(&mut *snake_fragment.position, &mut position);
                mem::swap(&mut *snake_fragment.direction.current, &mut direction);
            }
        }
    });
}

pub fn update_snake_sprite(
    snake_query: Query<&Snake>,
    mut query: Query<(&mut Handle<Image>, &SnakeFragment, DirectionQuery)>,
    textures: Res<TextureAssets>,
) {
    snake_query.for_each(|snake| {
        let mut iter = query.iter_many_mut(snake.fragments.iter().copied());

        while let Some((mut texture, snake_fragment, direction)) = iter.fetch_next() {
            let new_texture = match snake_fragment {
                SnakeFragment::Head => textures.snake_head.clone(),
                SnakeFragment::Tail => textures.snake_tail.clone(),
                SnakeFragment::Body(seed) => {
                    let fragment_textures = &textures.snake_fragment_assets[*seed as usize];

                    match (direction.current, direction.previous) {
                        (direction, Some(previous_direction)) if direction.clockwise() == **previous_direction => {
                            fragment_textures.right_curved.clone()
                        }
                        (direction, Some(previous_direction))
                            if direction.counter_clockwise() == **previous_direction =>
                        {
                            fragment_textures.left_curved.clone()
                        }
                        _ => fragment_textures.straight.clone(),
                    }
                }
            };

            *texture = new_texture;
        }
    });
}

pub fn update_snake_transform(
    snake_query: Query<&Snake>,
    mut query: Query<(&mut Transform, &SnakeFragment, DirectionQuery)>,
) {
    snake_query.for_each(|snake| {
        let mut iter = query.iter_many_mut(snake.fragments.iter().copied());

        while let Some((mut transform, snake_fragment, direction)) = iter.fetch_next() {
            let direction = match snake_fragment {
                SnakeFragment::Head | SnakeFragment::Tail => *direction.current,
                SnakeFragment::Body(_) => match (direction.current, direction.previous) {
                    (direction, Some(previous_direction)) if *direction != **previous_direction => **previous_direction,
                    (direction, _) => *direction,
                },
            };

            transform.rotation = direction.to_quat();
        }
    });
}

fn grow_snake(
    mut commands: Commands,
    mut snake_query: Query<(&mut Snake, &mut NumberGenerator<u16>)>,
    mut snake_fragment_query: Query<&mut SnakeFragment>,
    snake_position_query: Query<&Position, Without<Apple>>,
    apple_position_query: Query<(Entity, &Position), With<Apple>>,
) {
    snake_query.for_each_mut(|(mut snake, mut number_generator)| {
        let snake_head_position = snake_position_query.get(snake.fragments[0]).unwrap();

        for (apple_entity, apple_position) in apple_position_query.iter() {
            if *snake_head_position == *apple_position {
                commands.entity(apple_entity).despawn_recursive();

                let mut snake_tail_fragment = snake_fragment_query.get_mut(*snake.fragments.last().unwrap()).unwrap();
                *snake_tail_fragment = SnakeFragment::Body(number_generator.generate());

                let (position, direction) = snake.last_known_tail.take().unwrap();
                let snake_tail_entity = spawn_snake_fragment(&mut commands, position.0, direction, SnakeFragment::Tail);

                snake.fragments.push(snake_tail_entity);
            }
        }
    });
}

// fn check_snake_head_collision(snake_query: Query<>)

// fn stop_snake(mut commands: Commands, mut collision_reader: EventReader<DiscreteCollisionEvent<Snake,
// SnakeFragment>>) {     for collision in collision_reader.iter() {
//         commands.entity(collision.0).insert(Pause);
//     }
// }

pub fn spawn_snake(
    commands: &mut Commands,
    direction: Direction,
    position: IVec3,
    fragment_count: NonZeroUsize,
    mut number_generator: NumberGenerator<u16>,
) -> Entity {
    let offset = IVec3::from(direction.opposite());
    let positions = (0..fragment_count.get())
        .into_iter()
        .map(|i| position + offset * i as i32)
        .collect::<Vec<_>>();

    let fragments = positions
        .iter()
        .enumerate()
        .map(|(i, position)| {
            let fragment = match i {
                0 => SnakeFragment::Head,
                i if i < positions.len() - 1 => SnakeFragment::Body(number_generator.generate()),
                _ => SnakeFragment::Tail,
            };

            spawn_snake_fragment(commands, *position, direction, fragment)
        })
        .collect();

    commands
        .spawn()
        .insert_bundle((
            Snake {
                fragments,
                next_direction: None,
                last_known_tail: None,
            },
            number_generator,
            MoveTimer(Timer::from_seconds(0.125, true)),
            Name::new("Snake"),
        ))
        .id()
}

fn spawn_snake_fragment(
    commands: &mut Commands,
    position: IVec3,
    direction: Direction,
    fragment: SnakeFragment,
) -> Entity {
    commands
        .spawn_bundle(SpriteBundle::default())
        .insert_bundle((
            direction,
            fragment,
            Position(position),
            Name::new("Snake Fragment"),
            RigidBody,
        ))
        .id()
}
