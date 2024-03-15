pub mod opponent;
pub mod physics;
pub mod player;
pub mod prelude;
pub mod scene;
pub mod score;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use opponent::plugins::OpponentPlugin;
use physics::plugins::PhysicsPlugin;
use player::plugins::PlayerPlugin;
use scene::plugins::ScenePlugin;
use score::plugins::ScorePlugin;

pub struct PongPlugins;

impl PluginGroup for PongPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PhysicsPlugin)
            .add(ScenePlugin)
            .add(OpponentPlugin)
            .add(PlayerPlugin)
            .add(ScorePlugin)
    }
}
