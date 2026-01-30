use super::super::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Drag {
    pub drag_coefficient: f32,
}

impl Force for Drag {
    fn apply(&self, physical_body: &mut PhysicalBody) {
        physical_body.add_force(-self.drag_coefficient * physical_body.velocity);
    }
    fn get_force(&self, physical_body: &PhysicalBody) -> Vec3 {
        -self.drag_coefficient * physical_body.velocity
    }
}

impl Drag {
    #[allow(dead_code)]
    pub fn new(drag_coefficient: f32) -> Self {
        Self { drag_coefficient }
    }
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            drag_coefficient: 0.1,
        }
    }
}
