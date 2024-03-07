use bevy::prelude::*;
use crate::attack::AttackEvent;
use crate::targeting::Targeter;

#[derive(Debug, Copy, Clone)]
pub struct Weapon {
    pub damage: u32,
    pub range: f32,
    pub cooldown: f32,
    pub time_since_last_fired: f32,
    pub accuracy: f32,
}

impl Weapon {}

#[derive(Component, Debug)]
pub struct WeaponSlot {
    pub weapon: Weapon,
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,activate_weapon);
    }
}

fn activate_weapon(
    mut events: EventWriter<AttackEvent>,
    query: Query<(Entity, &Targeter, &Transform, &WeaponSlot)>
){
    for (entity, targeter, transform, weapon_slot) in query.iter() {
        if targeter.target.is_none() {
            continue;
        }
        events.send(AttackEvent {
            attacker: entity,
            target: targeter.target.unwrap(),
            weapon: weapon_slot.weapon,
            attacker_position: transform.translation,
        });
    }

}