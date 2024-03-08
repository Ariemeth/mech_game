use bevy::prelude::*;
use crate::weapons::DamageType;

#[derive(Component, Debug)]
pub struct Health {
    hp: i64,
}

impl Health {
    pub fn new(hp: i64) -> Self {
        Health { hp }
    }

    pub fn get_hp(&self) -> i64 {
        self.hp
    }

    pub fn damage(&mut self, damage: DamageType) {
        self.hp -= match damage {
            DamageType::Kinetic(amount) => amount as i64,
            DamageType::Energy(amount) => amount as i64,
            DamageType::Explosive(amount) => amount as i64,
        };
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_health);
    }
}

fn check_health(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in query.iter() {
        if health.hp <= 0 {
            println!("Entity {:?} has died", entity);
            commands.entity(entity).despawn_recursive();
            continue;
        }
    }
}