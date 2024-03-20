use bevy::prelude::*;

use bevy::input::common_conditions::input_toggle_active;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;

#[derive(Resource, PartialEq, Debug)]
pub enum GameState {
    Paused,
    Running,
}

#[derive(Event, Debug)]
pub struct StartGame;

#[derive(Event, Debug)]
pub struct ResetGame;

#[derive(Component, Debug)]
pub struct Ball;

#[derive(Component, Debug)]
pub struct Paddle;

#[derive(Component, Debug)]
pub struct Wall;

#[derive(Bundle)]
pub struct BallBundle {
    pub ball: Ball,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub restitution: Restitution,
    pub friction: Friction,
    pub liner_velocity: LinearVelocity,
    pub material_mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl Default for BallBundle {
    fn default() -> Self {
        Self {
            ball: Ball,
            rigid_body: RigidBody::Dynamic,
            collider: Collider::circle(20.0),
            restitution: Restitution::new(1.0),
            friction: Friction::new(0.0),
            liner_velocity: LinearVelocity::ZERO,
            material_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle::default(),
                material: Handle::default(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct PaddleBundle {
    pub paddle: Paddle,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub restitution: Restitution,
    pub friction: Friction,
    pub external_forces: ExternalForce,
    pub material_mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl Default for PaddleBundle {
    fn default() -> Self {
        Self {
            paddle: Paddle,
            rigid_body: RigidBody::Kinematic,
            collider: Collider::rectangle(25.0, 200.0),
            restitution: Restitution::new(1.0),
            friction: Friction::new(0.0),
            external_forces: ExternalForce::ZERO,
            material_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle::default(),
                material: Handle::default(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct WallBundle {
    pub wall: Wall,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub restitution: Restitution,
    pub friction: Friction,
    pub material_mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl Default for WallBundle {
    fn default() -> Self {
        Self {
            wall: Wall,
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(0.0, 0.0),
            restitution: Restitution::new(1.0),
            friction: Friction::new(0.0),
            material_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle::default(),
                material: Handle::default(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
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
    commands.spawn(BallBundle {
        material_mesh: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(20.0))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        collider: Collider::circle(20.0),
        ..default()
    });
}

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(PaddleBundle {
        material_mesh: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(25.0, 125.0))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(-550.0, 0.0, 0.0),
            ..default()
        },
        collider: Collider::rectangle(25.0, 125.0),
        ..default()
    });

    commands.spawn(PaddleBundle {
        material_mesh: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(25.0, 125.0))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(550.0, 0.0, 0.0),
            ..default()
        },
        collider: Collider::rectangle(25.0, 125.0),
        ..default()
    });
}

pub fn spawn_walls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(WallBundle {
        material_mesh: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(1280.0, 25.0))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 375.0, 0.0),
            ..default()
        },
        collider: Collider::rectangle(1280.0, 25.0),
        ..default()
    });

    commands.spawn(WallBundle {
        material_mesh: MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(1280.0, 25.0))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, -375.0, 0.0),
            ..default()
        },
        collider: Collider::rectangle(1280.0, 25.0),
        ..default()
    });
}

pub fn start_game(
    mut on_start: EventWriter<StartGame>,
    mut state: ResMut<GameState>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) && *state == GameState::Paused {
        *state = GameState::Running;
        on_start.send(StartGame);
    }
}

pub fn detect_goal(mut on_reset: EventWriter<ResetGame>, ball: Query<&Transform, With<Ball>>) {
    let ball = match ball.iter().next() {
        Some(ball) => ball,
        None => return,
    };

    if ball.translation.x.abs() > 640.0 {
        on_reset.send(ResetGame);
    }
}

pub fn reset_game(
    mut on_reset: EventReader<ResetGame>,
    (mut ball, mut paddles): (
        Query<(&mut Transform, &mut LinearVelocity), With<Ball>>,
        Query<&mut Transform, (With<Paddle>, Without<Ball>)>,
    ),
    mut state: ResMut<GameState>,
) {
    for _ in on_reset.read() {
        for mut ball in ball.iter_mut() {
            let transform = &mut ball.0;
            let velocity = &mut ball.1;

            transform.translation.x = 0.0;
            transform.translation.y = 0.0;

            velocity.x = 0.0;
            velocity.y = 0.0;
        }

        for mut paddle in paddles.iter_mut() {
            paddle.translation.y = 0.0;
        }

        *state = GameState::Paused;
    }
}

pub fn restart_game(mut on_reset: EventWriter<ResetGame>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        on_reset.send(ResetGame);
    }
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pong".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            PhysicsPlugins::default(),
        ))
        .add_systems(
            Startup,
            (spawn_camera, spawn_ball, spawn_paddles, spawn_walls).chain(),
        )
        .add_systems(Update, (start_game, detect_goal, reset_game, restart_game))
        .insert_resource(Gravity::ZERO)
        .insert_resource(GameState::Paused)
        .add_event::<StartGame>()
        .add_event::<ResetGame>();
    }
}
