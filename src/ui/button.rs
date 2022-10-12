use bevy::prelude::*;

pub struct ButtonTextures {
    normal: Handle<Image>,
    hover: Handle<Image>,
}


fn button_interaction(
    mut commands: Commands,
    mut query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    query.for_each_mut(|(interaction, action, mut color)| {
        match *interaction {
            Interaction::Clicked => {
            }
            Interaction::Hovered => {
            }
            Interaction::None => {
            }
        }
    });
}
