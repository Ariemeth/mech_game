use bevy::prelude::*;
use crate::{
    asset_loader::SceneAssets,
    movement::Velocity,
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Bundle)]
struct MechBundle {
    velocity: Velocity,
    model: SceneBundle,
}

#[derive(Component, Debug)]
pub struct Mech;

pub struct MechPlugin;

impl Plugin for MechPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_mech);
    }
}

fn spawn_mech(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        MechBundle {
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
