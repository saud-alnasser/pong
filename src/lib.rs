use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;

pub struct PongPlugins;

impl Plugin for PongPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".to_string(),
                resolution: (1280.0, 720.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(PhysicsPlugins::default())
        .add_systems(
            Startup,
            (
                systems::spawn_camera,
                systems::spawn_ball,
                systems::spawn_paddles,
                systems::spawn_walls,
                systems::assign_opponent,
                systems::assign_player,
                systems::kick_off,
            )
                .chain(),
        )
        .add_systems(FixedUpdate, systems::move_player)
        .add_systems(FixedUpdate, systems::move_opponent)
        .insert_resource(Gravity::ZERO);
    }
}

mod components {
    use bevy::prelude::*;

    #[derive(Component, Debug)]
    pub struct Ball;

    #[derive(Component, Debug)]
    pub struct Paddle;

    #[derive(Component, Debug)]
    pub struct Wall;

    #[derive(Component, Debug)]
    pub struct Opponent;

    #[derive(Component, Debug)]
    pub struct Player;
}

mod bundles {
    use super::components::*;
    use bevy::prelude::*;
    use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
    use bevy_xpbd_2d::prelude::*;

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
}

mod systems {
    use super::bundles::*;
    use super::components::*;
    use bevy::prelude::*;
    use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
    use bevy_xpbd_2d::parry::na::clamp;
    use bevy_xpbd_2d::prelude::*;
    use rand::Rng;

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

    pub fn assign_player(
        mut commands: Commands,
        paddles: Query<(Entity, &Transform), (With<Paddle>, Without<Opponent>)>,
    ) {
        let(paddle, _) = paddles .iter()
            .find(|(_, transform)| transform.translation.x > 0.0)
            .expect("failed to construct player paddle; no paddle found on the right side of the screen");

        commands.entity(paddle).insert(Player);
    }

    pub fn kick_off(mut ball: Query<&mut LinearVelocity, With<Ball>>) {
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
