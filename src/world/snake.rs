use std::collections::VecDeque;
use std::num::NonZeroUsize;

use bevy::prelude::*;
use rand::Rng;

use crate::physics::{Position, Direction, PreviousDirection};
use crate::assets::Assets;

// use super::board::CollisionEvent;
// use super::apple::Apple;

#[derive(Debug, Component, Reflect)]
pub struct SnakeHead {
    pub timer: Timer,
    #[reflect(ignore)]
    pub fragments: VecDeque<Entity>,
    pub seeds: Vec<usize>,
    pub next_tail: Entity,
}


#[derive(Debug, Copy, Clone, Default, Component, Reflect)]
#[reflect(Component)]
pub struct SnakeFragment;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_snake.label("move"))
            // .add_system(eat_apple.label("eat"))
            .add_system(render_snake.after("move"));

        // #[cfg(feature = "debug")]
        // {
        //     app.register_type::<Position>()
        //         .register_type::<Direction>()
        //         .register_type::<PreviousDirection>()
        //         .register_type::<Size>()
        //         .register_type::<SnakeHead>()
        //         .register_type::<SnakeFragment>();
        // }
    }
}

fn move_snake(
    time: Res<Time>,
    mut snakes: Query<(Entity, &mut SnakeHead)>,
    mut snake_fragments: Query<(&mut Position, &mut Direction, &mut PreviousDirection)>,
) {
    for (head_entity, mut snake) in snakes.iter_mut() {
        snake.timer.tick(time.delta());

        if snake.timer.just_finished() {
            let tail_entity = *snake.fragments.back().unwrap();

            if let Ok(
                [(mut head_position, head_direction, mut head_previous_direction), (mut tail_position, mut tail_direction, mut tail_previous_direction)],
            ) = snake_fragments.get_many_mut([head_entity, tail_entity])
            {
                *tail_direction = *head_direction;
                *tail_previous_direction = *head_previous_direction;
                *tail_position = *head_position;

                head_position.0 += IVec3::from(*head_direction);
                head_previous_direction.0 = *head_direction;

                snake.fragments.pop_back();
                snake.fragments.push_front(tail_entity);
            }
        }
    }
}

pub fn render_snake(
    snakes: Query<(Entity, &SnakeHead)>,
    mut snake_fragments: Query<(&mut Transform, &mut Handle<Image>, &Direction, &PreviousDirection)>,
    assets: Res<Assets>,
) {
    for (head, snake) in snakes.iter() {
        let mut iter = snake_fragments.iter_many_mut([head].into_iter().chain(snake.fragments.iter().copied()));
        let mut i = 0;

        while let Some((mut transform, mut texture, direction, previous_direction)) = iter.fetch_next() {
            let (new_texture, rotation) = match i {
                0 => (assets.snake_head_texture.clone(), Quat::from(previous_direction.0)),
                i if i == snake.fragments.len() => (assets.snake_tail_texture.clone(), Quat::from(*direction)),
                _ => {
                    let fragment_assets = &assets.snake_fragment_assets[snake.seeds[i - 1]];

                    match (*direction, previous_direction.0) {
                        (a, b) if a == b => (fragment_assets.straight_texture.clone(), Quat::from(*direction)),
                        (Direction::Up, Direction::Left)
                        | (Direction::Right, Direction::Up)
                        | (Direction::Down, Direction::Right)
                        | (Direction::Left, Direction::Down) => {
                            (
                                fragment_assets.right_curved_texture.clone(),
                                Quat::from(previous_direction.0),
                            )
                        }
                        _ => {
                            (
                                fragment_assets.left_curved_texture.clone(),
                                Quat::from(previous_direction.0),
                            )
                        }
                    }
                }
            };

            transform.rotation = rotation;
            *texture = new_texture;

            i += 1;
        }
    }
}

// fn eat_apple(
//     mut commands: Commands,
//     mut collision_reader: EventReader<CollisionEvent<SnakeHead, Apple>>,
//     mut snake: Query<&mut SnakeHead>,
//     mut snake_tail_query: Query<(&mut Position, &mut Direction, &mut PreviousDirection, &mut Visibility)>,
// ) {
//     for event in collision_reader.iter() {
//         // first of all, remove the collided apple
//         commands.entity(event.collided).despawn_recursive();

//         let mut snake = snake.get_mut(event.collider).unwrap();
//         let snake_tail = *snake.fragments.back().unwrap();

//         let [(
//             mut next_tail_position,
//             mut next_tail_direction,
//             mut next_tail_previous_direction,
//             mut next_tail_visibility,
//         ), (tail_position, _, tail_previous_direction, _)] =
//             snake_tail_query.get_many_mut([snake.next_tail, snake_tail]).unwrap();

//         next_tail_position.0 = tail_position.0 + IVec3::from(tail_previous_direction.0.opposite());
//         *next_tail_direction = tail_previous_direction.0;
//         *next_tail_previous_direction = *tail_previous_direction;
//         next_tail_visibility.is_visible = true;

//         let next_tail = snake.next_tail;

//         snake.fragments.push_back(next_tail);
//         snake.seeds.push(generate_seed());

//         snake.next_tail = spawn_next_snake_tail_fragment(&mut commands);
//     }
// }

pub fn generate_seed() -> usize {
    let mut rng = rand::thread_rng();

    rng.gen_range(0..6)
}

pub fn spawn_snake(
    commands: &mut Commands,
    direction: Direction,
    position: IVec3,
    fragment_count: NonZeroUsize,
) -> Entity {
    let offset = IVec3::from(direction.opposite());
    let positions = (1..fragment_count.get())
        .into_iter()
        .map(|i| position + offset * i as i32)
        .collect::<Vec<_>>();

    let snake = spawn_snake_fragment(commands, position, direction);

    let fragments = positions
        .iter()
        .map(|position| spawn_snake_fragment(commands, *position, direction))
        .collect();

    let next_tail = spawn_next_snake_tail_fragment(commands);

    commands
        .entity(snake)
        .insert(SnakeHead {
            fragments,
            next_tail,
            timer: Timer::from_seconds(0.2, true),
            seeds: positions.iter().map(|_| generate_seed()).collect(),
        })
        .insert(Name::new("Snake Head"))
        .id()
}

fn spawn_next_snake_tail_fragment(commands: &mut Commands) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            texture: Handle::default(),
            ..Default::default()
        })
        .insert(SnakeFragment)
        .insert(Position(IVec3::ZERO))
        .insert(Direction::Down)
        .insert(PreviousDirection(Direction::Down))
        .insert(Visibility { is_visible: false })
        .insert(Name::new("Snake Fragment"))
        .id()
}

fn spawn_snake_fragment(commands: &mut Commands, position: IVec3, direction: Direction) -> Entity {
    commands
        .spawn_bundle(SpriteBundle {
            texture: Handle::default(),
            ..Default::default()
        })
        .insert(SnakeFragment)
        .insert(Position(position))
        .insert(direction)
        .insert(PreviousDirection(direction))
        .insert(Name::new("Snake Fragment"))
        .id()
}
