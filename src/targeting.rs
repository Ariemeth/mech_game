use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Targeting {
    pub target: Option<Entity>,
    pub targeting_type: TargetingType,
}

#[derive(Debug)]
pub enum TargetingType {
    Closest
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
    mut query: Query<(&mut Targeting, &Transform)>,
    possible_targets: Query<(Entity, &Targetable, &Transform)>,
) {
    for (mut targeting, transform) in query.iter_mut() {
        if targeting.target.is_some() {
            println!("Already targeting {:?}", &targeting.target);
            continue;
        }
        match targeting.targeting_type {
            TargetingType::Closest => {
                let mut distance = f32::MAX;
                let mut possible_target = None;
                for (entity, _targetable, target_transform) in possible_targets.iter() {
                    if transform == target_transform {
                        println!("Cannot target oneself");
                        continue;
                    }
                    let distance_to_target = transform.translation.distance_squared(target_transform.translation);

                    if distance_to_target < distance {
                        println!("Targetable {:?} is closer", &entity);
                        possible_target = Some(entity);
                        distance = distance_to_target
                    }
                }

                if let Some(target) = possible_target {
                    targeting.target = Some(target);
                }
            }
        }
    }
}