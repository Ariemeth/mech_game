use bevy::prelude::*;

#[derive(Event)]
pub struct SteeringEvent {
    pub entity: Entity,
    pub steering_type: SteeringType,
}

#[derive(Component, Debug)]
pub struct SteeringBehavior {
    pub steering_type: SteeringType,
}

#[derive(Debug, Copy, Clone)]
#[derive(PartialEq)]
pub enum SteeringType {
    None,
    //   Seek(Entity),
    Direct(Vec3),
    //   Stationary,
    // Flee,
    // Arrive,
    // Wander,
    // Pursue,
    // Evade,
    // ObstacleAvoidance,
    // WallAvoidance,
    // Interpose,
    // Hide,
    // PathFollowing,
    // OffsetPursuit,
    // Separation,
    // Alignment,
    // Cohesion,
    // Flocking,
}

impl Default for SteeringBehavior {
    fn default() -> Self {
        SteeringBehavior {
            steering_type: SteeringType::None,
        }
    }
}

#[derive(Component, Debug)]
pub struct TargetingBehavior {
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
            steering: SteeringBehavior::default(),
            targeting: TargetingBehavior::default(),
        }
    }
}


pub struct BehaviorsPlugin;

impl Plugin for BehaviorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_steering, update_targeting));
        app.add_event::<SteeringEvent>();
    }
}

fn update_steering(
    mut events: EventReader<SteeringEvent>,
    mut entities: Query<&mut SteeringBehavior>,
) {
    for event in events.read() {
        if let Ok(mut behavior) = entities.get_mut(event.entity) {
            if behavior.steering_type == event.steering_type {
                continue;
            }
            println!("Updating steering for entity {:?} to {:?}", event.entity, event.steering_type);
            behavior.steering_type = event.steering_type.clone();
        }
    }
}

fn update_targeting() {}