use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::state::State;
use crate::{systems, Assets};

#[derive(Component)]
struct ButtonAction(Box<dyn Fn(&mut Commands) + Send + Sync + 'static>);

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct ButtonHover(Color);

#[derive(Component)]
struct ButtonNormal(Color);

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(State::MainMenu, setup_main_menu)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(State::MainMenu)
                    .with_system(start_game)
                    .with_system(exit_main_menu)
                    .with_system(button_interaction)
                    .into(),
            )
            .add_exit_system(State::MainMenu, systems::despawn_all);
    }
}

fn start_game(mut commands: Commands, query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>) {
    query.for_each(|interaction| {
        if matches!(*interaction, Interaction::Clicked) {
            commands.insert_resource(NextState(State::Game));
        }
    });
}

fn exit_main_menu(
    mut exit_writer: EventWriter<AppExit>,
    query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
) {
    for interaction in query.iter() {
        if matches!(*interaction, Interaction::Clicked) {
            exit_writer.send_default();
            break;
        }
    }
}

fn button_interaction(
    mut query: Query<(&mut UiColor, &Interaction, &ButtonNormal, &ButtonHover), (Changed<Interaction>, With<Button>)>,
) {
    query.for_each_mut(|(mut color, interaction, button_normal, button_hover)| {
        match *interaction {
            Interaction::None => {
                *color = button_normal.0.into();
            }
            Interaction::Hovered => {
                *color = button_hover.0.into();
            }
            _ => (),
        }
    });
}

fn setup_main_menu(mut commands: Commands, assets: Res<Assets>) {
    commands.spawn_bundle(Camera2dBundle::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,

                ..Default::default()
            },
            color: UiColor(Color::rgba(0., 0., 0., 0.)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            bottom: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: assets.title_font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(PlayButton)
                .insert(ButtonNormal(Color::rgb(0.1, 0.1, 0.1)))
                .insert(ButtonHover(Color::rgb(0.2, 0.2, 0.2)));

            parent
                .spawn_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect {
                            bottom: Val::Px(20.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: assets.title_font.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(ExitButton)
                .insert(ButtonNormal(Color::rgb(0.1, 0.1, 0.1)))
                .insert(ButtonHover(Color::rgb(0.2, 0.2, 0.2)));
        });
}
