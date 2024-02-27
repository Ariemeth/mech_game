use bevy::prelude::*;
use crate::mech::Mech;
use crate::movement::{Acceleration, Velocity};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

fn print_position(query: Query<(Entity, &Transform, &Acceleration, &Velocity),With<Mech>>) {
    // Log the entity ID and translation of each entity with a `Position` component.
    for (entity, transform, acceleration, velocity) in query.iter() {
        info!(
            "Entity {:?} pos: {:?}, vel: {:?}, acc: {:?}",
            entity, transform.translation,velocity.value, acceleration.value
        );
    }
}
