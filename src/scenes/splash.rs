use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::assets::Assets;
use crate::state::State;
use crate::systems;

#[derive(Component, Deref, DerefMut)]
pub struct SplashTimer(Timer);

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loopless_state(State::Splash)
            .init_resource::<Assets>()
            .add_enter_system(State::Splash, setup_splash)
            .add_system(change_state.run_in_state(State::Splash).run_if(timer_finished))
            .add_exit_system(State::Splash, systems::despawn_all);
    }
}

fn setup_splash(mut commands: Commands, assets: Res<Assets>) {
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
            parent.spawn_bundle(TextBundle::from_section(
                "ALP",
                TextStyle {
                    font: assets.title_font.clone(),
                    font_size: 200.,
                    color: Color::WHITE,
                },
            ));

            parent.spawn_bundle(
                TextBundle::from_section(
                    "production",
                    TextStyle {
                        font: assets.title_font.clone(),
                        font_size: 100.,
                        color: Color::WHITE,
                    },
                )
            );
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, false)));
}

fn timer_finished(time: Res<Time>, timer: Option<ResMut<SplashTimer>>) -> bool {
    timer
        .map(|mut timer| {
            timer.tick(time.delta());

            timer.finished()
        })
        .unwrap_or(false)
}

fn change_state(mut commands: Commands) {
    commands.insert_resource(NextState(State::MainMenu));
}
