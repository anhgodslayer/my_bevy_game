use bevy::prelude::*;

#[derive(Component)]
pub struct VelocityRotator {
    pub rotation_angle: f32,
    pub lerp_factor: f32,
    pub max_rotate_vel: f32,
}
#[derive(Component, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Gravity(pub f32);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gravity_system);
    }
}

fn gravity_system(mut physic_query: Query<(&mut Velocity, &Gravity)>, time: Res<Time>) {
    physic_query.iter_mut().for_each(|(mut velocity, gravity)| {
        // info!("Test physic");
        velocity.0.y -= gravity.0 * time.delta_seconds() + 10.0;
    });
}
