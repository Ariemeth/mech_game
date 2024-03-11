use bevy::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    health::Health,
    movement::Acceleration,
    movement::Velocity,
    targeting::{Targetable, Targeter},
};
use crate::behaviors::BehaviorsBundle;
use crate::steering::Steering;
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
    name: Name,
    behaviors: BehaviorsBundle,
    steering: Steering,
}

impl MechBundle {
    fn new(
        name: &str,
        scene_assets: &Res<SceneAssets>,
        starting_location: Vec3,
        starting_velocity: Vec3,
        angle: f32,
    ) -> MechBundle {
        let mut transform = Transform::from_translation(starting_location);
        transform.rotate_y(angle);

        MechBundle {
            name: Name::new(name.to_string()),
            acceleration: Acceleration::new(
                Vec3::ZERO,
                Vec3::new(2.4, 0., 2.4),
            ),
            velocity: Velocity::new (
                starting_velocity,
                Vec3::new(9.0, 0., 2.4),
            ),
            model: SceneBundle {
                scene: scene_assets.mech.clone(),
                transform,
                ..default()
            },
            mech: Mech,
            targeting: Targeter::default(),
            targetable: Targetable {},
            health: Health::new(100),
            behaviors: BehaviorsBundle::default(),
            steering: Steering::default(),
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
        "Mech 1",
        &scene_assets,
        Vec3::new(40.0, 0.0, 20.0),
        Vec3::ZERO,
        -std::f32::consts::PI / 2.0,
    );
    let mech1 = commands.spawn(mech_bundle_1).id();
    let mech1_weapon1 = commands.spawn(WeaponSlot {
        weapon: Weapon::new(DamageType::Kinetic(11), 100.0, 1.0, 0.85),
    }).id();
    commands.entity(mech1).push_children(&[mech1_weapon1]);


    let mech_bundle_2 = MechBundle::new(
        "Mech 2",
        &scene_assets,
        Vec3::new(-40.0, 0.0, 0.0),
        Vec3::ZERO,
        std::f32::consts::PI / 2.0,
    );
    let mech2 = commands.spawn(mech_bundle_2).id();
    let mech2_weapon1 = commands.spawn(WeaponSlot {
        weapon: Weapon::new(DamageType::Energy(10), 60.0, 1.0, 0.9),
    }).id();

    let mech2_weapon2 = commands.spawn(WeaponSlot {
        weapon: Weapon::new(DamageType::Explosive(5), 90.0, 1.5, 0.25),
    }).id();
    commands.entity(mech2).push_children(&[mech2_weapon1, mech2_weapon2]);
}
