use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct SteeringBehavior{
}

#[derive(Component, Debug)]
pub struct TargetingBehavior{
    pub targeting_type: TargetingType,
}

impl Default for TargetingBehavior {
    fn default() -> Self {
        TargetingBehavior {
            targeting_type: TargetingType::Closest,
        }
    }
}

#[derive(Debug)]
pub enum TargetingType {
    Closest
}

#[derive(Bundle)]
pub struct BehaviorsBundle {
    steering: SteeringBehavior,
    targeting: TargetingBehavior,
}

impl Default for BehaviorsBundle {
    fn default() -> Self {
        BehaviorsBundle {
            steering: SteeringBehavior {},
            targeting: TargetingBehavior::default(),
        }
    }
}


pub struct BehaviorsPlugin;

impl Plugin for BehaviorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_steering, update_targeting));
    }
}

fn update_steering(

) {

}

fn update_targeting(

) {

}