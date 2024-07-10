use bevy::app::AppExit;
use bevy::prelude::*;

pub struct GlobalInputPlugin;

impl Plugin for GlobalInputPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, exit_system);
    }
}

fn exit_system(
    mut exit: EventWriter<AppExit>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}