use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
    pub max: Vec3,
}

impl Velocity{
    pub fn new(starting_value: Vec3,max_value: Vec3)->Velocity{
        Velocity{
            value: starting_value,
            max: max_value,
        }
    }

    fn clamp(&mut self) -> &mut Velocity {
        self.value.x = self.value.x.min(self.max.x).max(-self.max.x);
        self.value.y = self.value.y.min(self.max.y).max(-self.max.y);
        self.value.z = self.value.z.min(self.max.z).max(-self.max.z);
        return self;
    }
}

#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
    pub max: Vec3,
}

impl Acceleration{
    pub fn new(starting_value: Vec3,max_value: Vec3)->Acceleration{
        Acceleration{
            value: starting_value,
            max: max_value,
        }
    }

    fn clamp(&mut self) -> &mut Acceleration {
        self.value.x = self.value.x.min(self.max.x).max(-self.max.x);
        self.value.y = self.value.y.min(self.max.y).max(-self.max.y);
        self.value.z = self.value.z.min(self.max.z).max(-self.max.z);
        return self;
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_velocity.before(update_position))
            .add_systems(Update, update_position);
    }
}

fn update_position(mut query: Query<(&mut Velocity, &mut Transform)>, time: Res<Time>) {
    for (mut velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.clamp().value * time.delta_seconds();
    }
}

fn update_velocity(mut query: Query<(&mut Velocity, &mut Acceleration)>, time: Res<Time>) {
    for (mut velocity, mut acceleration) in query.iter_mut() {
        velocity.value += acceleration.clamp().value * time.delta_seconds();
    }
}
