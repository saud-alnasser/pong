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
    use bevy::prelude::*;

    pub struct OpponentPlugin;

    impl Plugin for OpponentPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(FixedUpdate, systems::move_opponent);
        }
    }
}
