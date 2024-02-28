use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    movement::Acceleration,
    movement::Velocity,
};

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
        app.add_systems(PostStartup, spawn_mechs);
    }
}

fn spawn_mechs(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let mech_bundle_1 = build_mech(
        &scene_assets,
        Vec3::new(40.0, 0.0, 0.0),
        Vec3::ZERO,
        -std::f32::consts::PI / 2.0,
    );
    commands.spawn((
        mech_bundle_1,
        Mech,
    ));

    let mech_bundle_2 = build_mech(
        &scene_assets,
        Vec3::new(-40.0, 0.0, 0.0),
        Vec3::ZERO,
        std::f32::consts::PI / 2.0,
    );
    commands.spawn((
        mech_bundle_2,
        Mech,
    ));
}

fn build_mech(
    scene_assets: &Res<SceneAssets>,
    starting_location: Vec3,
    starting_velocity: Vec3,
    angle: f32,
) -> MechBundle {
    let mut transform = Transform::from_translation(starting_location);
    transform.rotate_y(angle);

    MechBundle {
        acceleration: Acceleration { value: Vec3::ZERO },
        velocity: Velocity {
            value: starting_velocity,
        },
        model: SceneBundle {
            scene: scene_assets.mech.clone(),
            transform,
            ..default()
        },
    }
}