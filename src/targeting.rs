use bevy::prelude::*;
use crate::behaviors::{TargetingBehavior, TargetingType};

#[derive(Component, Debug)]
pub struct Targeter {
    pub target: Option<Entity>,
}

impl Default for Targeter {
    fn default() -> Self {
        Targeter {
            target: None,
        }
    }
}


#[derive(Component, Debug, Copy, Clone)]
pub struct Targetable {}

pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_target);
    }
}

fn update_target(
    mut query: Query<(Entity, &mut Targeter, &Transform, &TargetingBehavior)>,
    possible_targets: Query<(Entity, &Targetable, &Transform)>,
) {
    for (targeting_entity, mut targeting, transform, behavior) in query.iter_mut() {
        if let Some(current_target) = targeting.target {
            // Check if the current target is still a viable target
            if possible_targets.contains(current_target) {
                continue;
            }
        }
        match behavior.targeting_type {
            TargetingType::Closest => {
                target_closest(&possible_targets, &targeting_entity, &mut targeting, transform);
            }
        }
    }
}

fn target_closest(
    possible_targets: &Query<(Entity, &Targetable, &Transform)>,
    targeting_entity: &Entity,
    targeting: &mut Mut<Targeter>, transform: &Transform
) {
    let mut distance = f32::MAX;
    let mut possible_target = None;
    for (entity, _targetable, target_transform) in possible_targets.iter() {
        if transform == target_transform {
            continue;
        }
        let distance_to_target = transform.translation.distance_squared(target_transform.translation);

        if distance_to_target < distance {
            println!("Targetable {:?} is closer to {:?}", &entity, &targeting_entity);
            possible_target = Some(entity);
            distance = distance_to_target
        }
    }

    match possible_target {
        Some(target) => {
            targeting.target = Some(target);
        }
        None => { targeting.target = None; }
    }
}