use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Steering {}


pub struct SteeringPlugin;

impl Plugin for SteeringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_steering);
    }
}

fn update_steering(
    mut query: Query<(&Steering, &mut Transform)>,
    time: Res<Time>,
) {
    for (_steering, mut transform) in query.iter_mut() {
        transform.translation += Vec3::new(0.0, 0.0, 0.0) * time.delta_seconds();
    }
}