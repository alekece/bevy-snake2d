use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub enum Action {
    ChangeDirection,
}

pub struct InputController;

impl Plugin for InputController {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
    }
}
