use bevy::prelude::*;
use bevy_xpbd_2d::resources::Gravity;

use super::commands::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_camera, spawn_ball, spawn_paddles, spawn_walls),
        )
        .insert_resource(Gravity::ZERO);
    }
}
