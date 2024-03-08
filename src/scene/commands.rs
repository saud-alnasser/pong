use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use super::components::*;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materiels: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(20.0))),
            material: materiels.add(Color::WHITE),
            ..default()
        },
        Ball,
    ));
}

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(25.0, 200.0))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(-500.0, 0.0, 0.0),
            ..default()
        },
        Paddle,
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(25.0, 200.0))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(500.0, 0.0, 0.0),
            ..default()
        },
        Paddle,
    ));
}
