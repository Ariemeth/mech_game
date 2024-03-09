use bevy::prelude::*;

pub enum SteeringBehavior{

}
#[derive(Event)]
pub struct SteeringEvent{}

#[derive(Component, Debug)]
pub struct Steering {

}


pub struct SteeringPlugin;

impl Plugin for SteeringPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SteeringEvent>();
    }
}

fn update_steering(
    mut steering_events: EventReader<SteeringEvent>
) {

}