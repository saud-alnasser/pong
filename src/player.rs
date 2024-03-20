use crate::{
    opponent::{assign_opponent, Opponent},
    scene::{Paddle, StartGame},
};

use bevy::prelude::*;
use bevy_xpbd_2d::parry::na::clamp;

#[derive(Component, Debug)]
pub struct Player;

pub fn assign_player(
    mut on_start: EventReader<StartGame>,
    mut commands: Commands,
    paddles: Query<Entity, (With<Paddle>, Without<Opponent>)>,
) {
    for _ in on_start.read() {
        let paddle = paddles
            .iter()
            .next()
            .expect("failed to find a paddle to assign as a player");

        commands.entity(paddle).insert(Player);
    }
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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, assign_player.before(assign_opponent))
            .add_systems(FixedUpdate, move_player);
    }
}
