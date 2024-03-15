use bevy::prelude::*;
use pong::*;

fn main() {
    App::new().add_plugins(PongPlugins).run();
}
