use bevy::prelude::*;
use crate::health::Health;
use crate::weapons::Weapon;

#[derive(Event)]
pub struct AttackEvent {
    pub attacker: Entity,
    pub target: Entity,
    pub weapon: Weapon,
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
    mut query: Query<(&mut Health, &Transform)>,
    mut attack_events: EventReader<AttackEvent>,
) {
    for event in attack_events.read() {
        if let Ok((mut health, transform)) = query.get_mut(event.target) {
            let distance = transform.translation.distance(event.attacker_position);
            if distance < event.weapon.range {
                println!("Entity {:?} was attacked by {:?} for {:?} damage", event.target, event.attacker, event.weapon.damage);
                health.damage(event.weapon.damage);
            }
        }
    }
}

