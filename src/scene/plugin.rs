use bevy::prelude::*;

use super::commands::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_ball, spawn_paddles));
    }
}
