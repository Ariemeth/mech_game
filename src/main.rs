use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};

use camera::CameraPlugin;
use global_input::GlobalInputPlugin;
use mech::MechPlugin;
//use debug::DebugPlugin;
use movement::MovementPlugin;

use crate::asset_loader::AssetLoaderPlugin;
use crate::attack::AttackPlugin;
use crate::behaviors::BehaviorsPlugin;
use crate::health::HealthPlugin;
use crate::steering::SteeringPlugin;
use crate::targeting::TargetingPlugin;
use crate::weapons::WeaponPlugin;

mod camera;
mod debug;
mod movement;
mod mech;
mod global_input;
mod asset_loader;
mod targeting;
mod weapons;
mod health;
mod attack;
mod equipment;
mod steering;
mod behaviors;

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.,
        })
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(Backends::VULKAN),
                ..default()
            }),
            synchronous_pipeline_compilation: false,
        }), )
        // User defined plugins.
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        // .add_plugins(DebugPlugin)
        .add_plugins(MechPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GlobalInputPlugin)
        .add_plugins(TargetingPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(AttackPlugin)
        .add_plugins(BehaviorsPlugin)
        .add_plugins(SteeringPlugin)
        .run();
}
