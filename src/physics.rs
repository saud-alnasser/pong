pub use bevy_xpbd_2d::*;

pub mod plugins {
    use bevy::prelude::*;
    use bevy_xpbd_2d::prelude::*;

    pub struct PhysicsPlugin;

    impl Plugin for PhysicsPlugin {
        fn build(&self, app: &mut bevy::app::App) {
            app.add_plugins(PhysicsPlugins::default());
        }
    }
}
