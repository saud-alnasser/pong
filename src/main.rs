use bevy::prelude::*;
use pong::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default(), ScenePlugin))
        .run();
}
