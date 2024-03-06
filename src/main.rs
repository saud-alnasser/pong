use bevy::prelude::*;
use pong::prelude::*;

fn main() {
    App::new()
        .add_plugins(ScenePlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
