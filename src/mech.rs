use bevy::prelude::*;
use crate::{
    asset_loader::SceneAssets,
    movement::Velocity,
};
use crate::movement::Acceleration;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_SPEED: f32 = 25.0;

#[derive(Bundle)]
struct MechBundle {
    acceleration: Acceleration,
    velocity: Velocity,
    model: SceneBundle,
}

#[derive(Component, Debug)]
pub struct Mech;

pub struct MechPlugin;

impl Plugin for MechPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_mech)
            .add_systems(Update, mech_movement_controls);
    }
}

fn spawn_mech(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MechBundle {
            acceleration: Acceleration {value: Vec3::ZERO},
            velocity: Velocity {
                value: STARTING_VELOCITY,
            },
            model: SceneBundle {
                scene: scene_assets.mech.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Mech,
    ));
}

fn mech_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity, &mut Acceleration), With<Mech>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity, mut acceleration)) = query.get_single_mut() else {
        return;
    };
    let mut rotation = 0.0;
    let mut movement = 0.0;
    let mut acceleration_x = 0.0;

    if keyboard_input.pressed(KeyCode::KeyD) {
        acceleration_x = -1.;
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        acceleration_x = 1.;
    }

    acceleration.value.x = acceleration_x

    // if keyboard_input.pressed(KeyCode::KeyS) {
    //     movement = -SPACESHIP_SPEED;
    // } else if keyboard_input.pressed(KeyCode::KeyW) {
    //     movement = SPACESHIP_SPEED;
    // }

  //  transform.rotate_y(rotation);

    // Update the spaceship's velocity based on new direction.
   // velocity.value = -transform.forward() * movement;
}
