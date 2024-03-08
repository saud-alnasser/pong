use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::prelude::*;

#[derive(Bundle)]
struct BallBundle {
    ball: Ball,
    rigid_body: RigidBody,
    collider: Collider,
    restitution: Restitution,
    friction: Friction,
    material_mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl BallBundle {
    fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        position: (f32, f32, f32),
        radius: f32,
    ) -> Self {
        Self {
            ball: Ball,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::circle(radius),
            restitution: Restitution::new(1.0),
            friction: Friction::new(0.0),
            material_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(radius))),
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(position.0, position.1, position.2),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
struct PaddleBundle {
    paddle: Paddle,
    rigid_body: RigidBody,
    collider: Collider,
    restitution: Restitution,
    friction: Friction,
    material_mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl PaddleBundle {
    fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        position: (f32, f32, f32),
        size: (f32, f32),
    ) -> Self {
        Self {
            paddle: Paddle,
            rigid_body: RigidBody::Kinematic,
            collider: Collider::rectangle(size.0, size.1),
            restitution: Restitution::new(1.0),
            friction: Friction::new(0.0),
            material_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(size.0, size.1))),
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(position.0, position.1, position.2),
                ..default()
            },
        }
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((BallBundle::new(
        &mut meshes,
        &mut materials,
        (0.0, 0.0, 0.0),
        20.0,
    ),));
}

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((PaddleBundle::new(
        &mut meshes,
        &mut materials,
        (-500.0, 0.0, 0.0),
        (25.0, 200.0),
    ),));

    commands.spawn((PaddleBundle::new(
        &mut meshes,
        &mut materials,
        (500.0, 0.0, 0.0),
        (25.0, 200.0),
    ),));
}
