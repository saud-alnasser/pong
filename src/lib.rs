mod ball;
mod opponent;
mod player;
mod scene;

use bevy::prelude::*;

use ball::BallPlugin;
use opponent::OpponentPlugin;
use player::PlayerPlugin;
use scene::ScenePlugin;

pub struct PongPlugins;

impl Plugin for PongPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((ScenePlugin, OpponentPlugin, PlayerPlugin, BallPlugin));
    }
}
