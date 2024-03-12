pub mod physics;
pub mod prelude;
pub mod scene;

use bevy::app::{PluginGroup, PluginGroupBuilder};
use physics::plugins::PhysicsPlugin;
use scene::plugins::ScenePlugin;

pub struct PongPlugins;

impl PluginGroup for PongPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PhysicsPlugin)
            .add(ScenePlugin)
    }
}
