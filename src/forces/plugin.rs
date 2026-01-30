use super::*;
use bevy::prelude::*;

pub struct ForcesPlugin;

impl Plugin for ForcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_acceleration, update_velocity, update_position),
        );
    }
}

fn update_acceleration(mut query: Query<&mut PhysicalBody>) {
    for mut body in query.iter_mut() {
        body.acceleration = body.force / body.mass;
    }
}

fn update_velocity(mut query: Query<&mut PhysicalBody>, time: Res<Time>) {
    for mut body in query.iter_mut() {
        body.velocity = body.velocity + body.acceleration * time.delta_secs();
    }
}

fn update_position(mut query: Query<(&mut Transform, &PhysicalBody)>, time: Res<Time>) {
    for (mut transform, body) in query.iter_mut() {
        transform.translation = transform.translation + body.velocity * time.delta_secs();
    }
}
