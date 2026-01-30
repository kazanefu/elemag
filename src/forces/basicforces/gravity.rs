use super::super::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Gravity {
    pub acceleration: Vec3,
}

#[allow(dead_code)]
impl Gravity {
    pub fn new(scale: f32, direction: Vec3) -> Self {
        Self {
            acceleration: direction.normalize() * scale,
        }
    }
    pub fn new_earth() -> Self {
        Self {
            acceleration: Vec3::new(0.0, -9.81, 0.0),
        }
    }
    pub fn new_down(scale: f32) -> Self {
        Self::new(scale, Vec3::NEG_Y)
    }
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            acceleration: Vec3::new(0.0, -9.81, 0.0),
        }
    }
}

impl Force for Gravity {
    fn apply(&self, physical_body: &mut PhysicalBody) {
        physical_body.add_force(self.acceleration * physical_body.mass);
    }
    fn get_force(&self, physical_body: &PhysicalBody) -> Vec3 {
        self.acceleration * physical_body.mass
    }
}
