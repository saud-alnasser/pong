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

    pub fn assign_player(
        mut commands: Commands,
        paddles: Query<(Entity, &Transform), (With<Paddle>, Without<Opponent>)>,
    ) {
        let (paddle, _) = paddles.get_single().expect(
            "failed to construct player paddle; no paddle found available for player input",
        );

        commands.entity(paddle).insert(Player);
    }

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
    use crate::{opponent, scene};
    use bevy::prelude::*;

    pub struct PlayerPlugin;

    impl Plugin for PlayerPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(
                Startup,
                systems::assign_player
                    .after(scene::systems::spawn_paddles)
                    .after(opponent::systems::assign_opponent),
            )
            .add_systems(FixedUpdate, systems::move_player);
        }
    }
}
