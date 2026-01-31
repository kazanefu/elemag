use bevy::prelude::*;
mod forces;
use forces::defaultplugin::DefaultForcesPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultForcesPlugin)
        .run();
}
