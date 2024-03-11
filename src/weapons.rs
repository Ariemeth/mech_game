use std::time::Duration;

use bevy::prelude::*;

use crate::attack::AttackEvent;
use crate::behaviors::{SteeringEvent, SteeringType};
use crate::targeting::{Targetable, Targeter};

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
            return self.damage.zero_damage();
        }

        match self.damage {
            DamageType::Kinetic(amount) => {
                DamageType::Kinetic((amount as f32 * self.accuracy) as u32)
            }
            DamageType::Energy(amount) => {
                DamageType::Energy((amount as f32 * self.accuracy) as u32)
            }
            DamageType::Explosive(amount) => {
                DamageType::Explosive((amount as f32 * self.accuracy) as u32)
            }
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
    mut attack_events: EventWriter<AttackEvent>,
    mut steering_events: EventWriter<SteeringEvent>,
    mut query: Query<(&Parent, &mut WeaponSlot)>,
    mut query2: Query<(&Parent, &Children)>,
    parent_query: Query<(&Targeter, &Transform, Option<&Name>), With<Children>>,
    query_target: Query<(&Transform, Option<&Name>), With<Targetable>>,
) {


    for (parent, mut weapon_slot) in query.iter_mut() {
        let (targeter, attacker_transform, attacker_name) = if let Ok(targeter) = parent_query.get(parent.get()) {
            targeter
        } else {
            continue;
        };

        if targeter.target.is_none() {
            continue;
        }
        if !weapon_slot.weapon.weapon_ready() {
            continue;
        }

        let attacker_name = match attacker_name {
            Some(name) => name.to_string(),
            _ => String::from("Unknown"),
        };

        if let Ok((target_transform, target_name)) = query_target.get(targeter.target.unwrap()) {
            let distance = attacker_transform.translation.distance(target_transform.translation);

            if distance > weapon_slot.weapon.range {
                steering_events.send(SteeringEvent {
                    entity: parent.get(),
                    steering_type: SteeringType::Direct(target_transform.translation),
                });
                continue;
            }

            let damage = weapon_slot.weapon.attack(distance);

            println!("{:?} is attacking {:?} with {:?} for {} damage", attacker_name, match target_name {
                Some(name) => name.to_string(),
                _ => String::from("Unknown"),
            }, weapon_slot.weapon.damage, damage);

            attack_events.send(AttackEvent {
                attacker: parent.get(),
                target: targeter.target.unwrap(),
                damage,
                attacker_position: attacker_transform.translation,
            });
        }
    }
}