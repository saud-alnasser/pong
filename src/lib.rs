pub mod physics;
pub mod prelude;
pub mod scene;

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub struct PongPlugins;

impl PluginGroup for PongPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(physics::plugins::PhysicsPlugin)
            .add(scene::plugins::ScenePlugin)
    }
}
