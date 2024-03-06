use bevy::prelude::*;
use crate::targeting::Targeter;


pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,activate_weapon);
    }
}

fn activate_weapon(
    query: Query<(&Targeter, &Transform)>
){
    for (targeter, transform) in query.iter() {
        if targeter.target.is_none() {
            continue;
        }
    }


}