mod camera;
mod debug;
mod movement;
mod mech;
mod global_input;
mod asset_loader;

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use mech::MechPlugin;
use global_input::GlobalInputPlugin;
use crate::asset_loader::AssetLoaderPlugin;

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
        }),)
        // User defined plugins.
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(MechPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(GlobalInputPlugin)
        .run();
}
