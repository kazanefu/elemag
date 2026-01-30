pub mod basicforces;
pub mod defaultplugin;
pub mod plugin;

use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy)]
pub struct PhysicalBody {
    pub force: Vec3,
    pub mass: f32,
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

impl PhysicalBody {
    #[allow(dead_code)]
    pub fn add_force(&mut self, force: Vec3) {
        self.force = self.force + force;
    }
}

pub trait Force {
    fn apply(&self, physical_body: &mut PhysicalBody);
    #[allow(dead_code)]
    fn get_force(&self, physical_body: &PhysicalBody) -> Vec3;
}

#[allow(dead_code)]
pub fn register_force<T: Force + Component>(app: &mut App) {
    app.add_systems(Update, apply_force_system::<T>);
}
fn apply_force_system<T: Force + Component>(mut query: Query<(&mut PhysicalBody, &T)>) {
    for (mut body, force) in query.iter_mut() {
        force.apply(&mut body);
    }
}
