pub mod prelude {
    pub use super::components::*;
    pub use super::plugins::*;
}

pub mod components {
    use bevy::prelude::*;

    #[derive(Component, Debug)]
    pub struct Opponent;
}

pub mod systems {
    use crate::prelude::*;
    use bevy::prelude::*;
    use bevy_xpbd_2d::parry::na::clamp;

    pub fn assign_opponent(
        mut commands: Commands,
        paddles: Query<(Entity, &Transform), With<Paddle>>,
    ) {
        let (paddle, _) = paddles
            .iter()
            .find(|(_, transform)| transform.translation.x < 0.0)
            .expect("failed to construct opponent paddle; no paddle found on the left side of the screen");

        commands.entity(paddle).insert(Opponent);
    }

    pub fn move_opponent(
        (ball, mut paddle): (
            Query<&Transform, With<Ball>>,
            Query<&mut Transform, (With<Opponent>, Without<Ball>)>,
        ),
        time: Res<Time>,
    ) {
        let ball = match ball.iter().next() {
            Some(ball) => ball,
            None => return,
        };

        let mut paddle = match paddle.iter_mut().next() {
            Some(paddle) => paddle,
            None => return,
        };

        let speed = 350.0;
        let distance = match ball.translation.x < 0.0 {
            true => ball.translation.y - paddle.translation.y,
            false => 0.0 - paddle.translation.y,
        };

        let direction = distance.signum();
        let magnitude = (speed * time.delta_seconds()).min(distance.abs());

        paddle.translation.y += magnitude * direction;
        paddle.translation.y = clamp(paddle.translation.y, -275.0, 275.0);
    }
}

pub mod plugins {
    use super::systems;
    use crate::scene;
    use bevy::prelude::*;

    pub struct OpponentPlugin;

    impl Plugin for OpponentPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(
                Startup,
                systems::assign_opponent.after(scene::systems::spawn_paddles),
            )
            .add_systems(FixedUpdate, systems::move_opponent);
        }
    }
}
