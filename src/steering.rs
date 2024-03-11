use bevy::prelude::*;

use crate::behaviors::{SteeringBehavior, SteeringType};
use crate::movement::{Acceleration, Velocity};

#[derive(Component, Debug, Default)]
pub struct Steering;


pub struct SteeringPlugin;

impl Plugin for SteeringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_steering);
    }
}

fn update_steering(
    mut query: Query<(&SteeringBehavior, &mut Transform, &mut Velocity, &mut Acceleration), With<Steering>>,
) {
    for (behavior, mut transform, mut velocity, mut acceleration) in query.iter_mut() {
        match behavior.steering_type {
            SteeringType::None => {
                acceleration.value = Vec3::ZERO;
                velocity.value = Vec3::ZERO;
            }

            SteeringType::Direct(target) => {
                let direction = target - transform.translation;
                let direction = direction.normalize();
                acceleration.value += direction * 0.1;
                transform.look_to(-target, Vec3::Y);
            }
        }
    }
}