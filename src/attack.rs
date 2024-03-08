use bevy::prelude::*;
use crate::health::Health;
use crate::weapons::DamageType;

#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub damage: DamageType,
    pub attacker_position: Vec3,
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_attack);
        app.add_event::<AttackEvent>();
    }
}

fn apply_attack(
    mut query: Query<&mut Health>,
    mut attack_events: EventReader<AttackEvent>,
) {
    for event in attack_events.read() {
        if let Ok(mut health) = query.get_mut(event.target) {
            health.damage(event.damage);
            println!(
                "Entity {:?} took {} damage, remaining health: {}",
                event.target, event.damage, health.get_hp()
            );
        }
    }
}

