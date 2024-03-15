pub mod prelude {
    pub use super::components::*;
    pub use super::plugins::*;
}

pub mod components {
    use bevy::prelude::*;

    #[derive(Component, Debug)]
    pub struct Player;
}

pub mod systems {
    use crate::prelude::*;
    use bevy::prelude::*;
    use bevy_xpbd_2d::parry::na::clamp;

    pub fn move_player(
        mut player: Query<&mut Transform, (With<Paddle>, With<Player>)>,
        time: Res<Time>,
        keyboard: Res<ButtonInput<KeyCode>>,
    ) {
        let mut player = match player.iter_mut().next() {
            Some(player) => player,
            None => return,
        };

        let speed = 500.0;
        let mut direction = 0.0;

        if keyboard.pressed(KeyCode::KeyW) {
            direction = 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction = -1.0;
        }

        player.translation.y += direction * speed * time.delta_seconds();
        player.translation.y = clamp(player.translation.y, -275.0, 275.0)
    }
}

pub mod plugins {
    use super::systems;
    use bevy::prelude::*;

    pub struct PlayerPlugin;

    impl Plugin for PlayerPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(FixedUpdate, systems::move_player);
        }
    }
}
