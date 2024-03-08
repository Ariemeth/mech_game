use std::time::Duration;
use bevy::prelude::*;

use crate::attack::AttackEvent;
use crate::targeting::Targeter;

#[derive(Debug, Copy, Clone)]
pub enum DamageType {
    Kinetic(u32),
    Energy(u32),
    Explosive(u32),
}

impl DamageType {
    pub fn zero_damage(&self) -> DamageType {
        match self {
            DamageType::Kinetic(_) => DamageType::Kinetic(0),
            DamageType::Energy(_) => DamageType::Energy(0),
            DamageType::Explosive(_) => DamageType::Explosive(0),
        }
    }
}

impl std::fmt::Display for DamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DamageType::Kinetic(amount) => write!(f, "Kinetic({})", amount),
            DamageType::Energy(amount) => write!(f, "Energy({})", amount),
            DamageType::Explosive(amount) => write!(f, "Explosive({})", amount),
        }
    }
}

#[derive(Debug)]
pub struct Weapon {
    pub damage: DamageType,
    pub range: f32,
    pub cooldown: f32,
    pub time_since_last_fired: Timer,
    pub accuracy: f32,
}

impl Weapon {
    pub fn new(damage: DamageType, range: f32, cooldown: f32, accuracy: f32) -> Self {
        Weapon {
            damage,
            range,
            cooldown,
            time_since_last_fired: Timer::new(Duration::from_secs_f32(cooldown), TimerMode::Once),
            accuracy,
        }
    }
    pub fn weapon_ready(&self) -> bool {
        self.time_since_last_fired.finished()
    }
    pub fn attack(&mut self, range_to_target: f32) -> DamageType {
        self.time_since_last_fired = Timer::new(Duration::from_secs_f32(self.cooldown), TimerMode::Once);
        if range_to_target > self.range {
            return self.damage.zero_damage()
        }

        match self.damage {
            DamageType::Kinetic(amount) => {
                DamageType::Kinetic((amount as f32 * self.accuracy) as u32)
            },
            DamageType::Energy(amount) => {
                DamageType::Energy((amount as f32 * self.accuracy) as u32)
            },
            DamageType::Explosive(amount) => {
                DamageType::Explosive((amount as f32 * self.accuracy) as u32)
            },
        }
    }
}

#[derive(Component, Debug)]
pub struct WeaponSlot {
    pub weapon: Weapon,
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, activate_weapon)
            .add_systems(Update, weapon_tick);
    }
}

fn weapon_tick(
    time: Res<Time>,
    mut query: Query<&mut WeaponSlot>,
) {
    for mut weapon_slot in query.iter_mut() {
        weapon_slot.weapon.time_since_last_fired.tick(time.delta());
    }
}

fn activate_weapon(
    mut events: EventWriter<AttackEvent>,
    mut query: Query<(Entity, &Targeter, &Transform, &mut WeaponSlot)>,
    query_target: Query<&Transform>,
) {
    for (entity, targeter, attacker_transform, mut weapon_slot) in query.iter_mut() {
        if targeter.target.is_none() {
            continue;
        }
        if !weapon_slot.weapon.weapon_ready() {
            continue;
        }

        if let Ok(target_transform) = query_target.get(targeter.target.unwrap()) {
            let distance = attacker_transform.translation.distance(target_transform.translation);
            let damage = weapon_slot.weapon.attack(distance);

            events.send(AttackEvent {
                attacker: entity,
                target: targeter.target.unwrap(),
                damage,
                attacker_position: attacker_transform.translation,
            });
        }
    }
}