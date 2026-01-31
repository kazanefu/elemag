use super::*;
use basicforces::*;
use bevy::prelude::*;

#[allow(dead_code)]
pub struct DefaultForcesPlugin;

impl Plugin for DefaultForcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(plugin::ForcesPlugin);
        register_force::<gravity::Gravity>(app);
        register_force::<drag::Drag>(app);
        app.add_plugins(point_charge::PointChargePlugin);
        register_force::<point_charge::PointCharge>(app);
    }
}
