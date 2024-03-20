use crate::{
    player::Player,
    scene::{Ball, Paddle, StartGame},
};

use bevy::prelude::*;
use bevy_xpbd_2d::parry::na::clamp;

#[derive(Component, Debug)]
pub struct Opponent;

pub fn assign_opponent(
    mut on_start: EventReader<StartGame>,
    mut commands: Commands,
    paddles: Query<Entity, (With<Paddle>, Without<Player>)>,
) {
    for _ in on_start.read() {
        let paddle = paddles
            .iter()
            .next()
            .expect("failed to find a paddle to assign as an opponent");

        commands.entity(paddle).insert(Opponent);
    }
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
    let distance = match paddle.translation.x < 0.0 {
        true => match ball.translation.x < 0.0 {
            true => ball.translation.y - paddle.translation.y,
            false => 0.0 - paddle.translation.y,
        },
        false => match ball.translation.x > 0.0 {
            true => ball.translation.y - paddle.translation.y,
            false => 0.0 - paddle.translation.y,
        },
    };

    let direction = distance.signum();
    let magnitude = (speed * time.delta_seconds()).min(distance.abs());

    paddle.translation.y += magnitude * direction;
    paddle.translation.y = clamp(paddle.translation.y, -275.0, 275.0);
}

pub struct OpponentPlugin;

impl Plugin for OpponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, assign_opponent)
            .add_systems(FixedUpdate, move_opponent);
    }
}
