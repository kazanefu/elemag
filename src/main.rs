use bevy::prelude::*;
mod forces;
use forces::plugin::ForcesPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ForcesPlugin)
        .run();
}
