use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    health::Health,
    movement::Acceleration,
    movement::Velocity,
    targeting::{Targetable, Targeter},
};
use crate::targeting::TargetingType;
use crate::weapons::{DamageType, Weapon, WeaponSlot};

#[derive(Bundle)]
struct MechBundle {
    acceleration: Acceleration,
    velocity: Velocity,
    model: SceneBundle,
    mech: Mech,
    targeting: Targeter,
    targetable: Targetable,
    health: Health,
}

impl MechBundle {
    fn new(
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
            mech: Mech,
            targeting: Targeter {
                target: None,
                targeting_type: TargetingType::Closest,
            },
            targetable: Targetable {},
            health: Health::new(100),
        }
    }
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
    let mech_bundle_1 = MechBundle::new(
        &scene_assets,
        Vec3::new(40.0, 0.0, 0.0),
        Vec3::ZERO,
        -std::f32::consts::PI / 2.0,
    );
    commands.spawn((
        mech_bundle_1,
        WeaponSlot {
            weapon: Weapon::new(DamageType::Kinetic(11), 100.0, 1.0, 0.85),
        }
    ));

    let mech_bundle_2 = MechBundle::new(
        &scene_assets,
        Vec3::new(-40.0, 0.0, 0.0),
        Vec3::ZERO,
        std::f32::consts::PI / 2.0,
    );
    commands.spawn((
        mech_bundle_2,
        WeaponSlot {
            weapon: Weapon::new(DamageType::Energy(10), 100.0, 1.0, 0.9),
        }));
}

