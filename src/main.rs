use bevy::prelude::*;
use pong::PongPlugins;

fn main() {
    App::new().add_plugins((DefaultPlugins, PongPlugins)).run();
}
