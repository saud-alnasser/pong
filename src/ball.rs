use crate::scene::{Ball, StartGame};

use bevy::prelude::*;
use bevy_xpbd_2d::components::LinearVelocity;
use rand::Rng;

pub fn kick_off(
    mut on_start: EventReader<StartGame>,
    mut ball: Query<&mut LinearVelocity, With<Ball>>,
) {
    for _ in on_start.read() {
        let mut ball_velocity = match ball.iter_mut().next() {
            Some(ball) => ball,
            None => return,
        };

        let velocity = {
            let mut rng = rand::thread_rng();

            let speed = 500.0;

            let x_direction = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            let y_direction = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

            Vec2::new(x_direction, y_direction) * speed
        };

        ball_velocity.x = velocity.x;
        ball_velocity.y = velocity.y;
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, kick_off);
    }
}
