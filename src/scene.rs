pub mod prelude {
    pub use super::components::*;
    pub use super::plugins::*;
}

pub mod components {
    use bevy::prelude::*;

    #[derive(Component, Debug)]
    pub struct Ball;

    #[derive(Component, Debug)]
    pub struct Paddle;

    #[derive(Component, Debug)]
    pub struct Wall;
}

pub mod systems {
    use crate::prelude::*;
    use bevy::prelude::*;
    use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
    use rand::Rng;

    #[derive(Bundle)]
    struct BallBundle {
        ball: Ball,
        rigid_body: RigidBody,
        collider: Collider,
        restitution: Restitution,
        friction: Friction,
        liner_velocity: LinearVelocity,
        material_mesh: MaterialMesh2dBundle<ColorMaterial>,
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
    struct PaddleBundle {
        paddle: Paddle,
        rigid_body: RigidBody,
        collider: Collider,
        restitution: Restitution,
        friction: Friction,
        external_forces: ExternalForce,
        material_mesh: MaterialMesh2dBundle<ColorMaterial>,
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
    struct WallBundle {
        wall: Wall,
        rigid_body: RigidBody,
        collider: Collider,
        restitution: Restitution,
        friction: Friction,
        material_mesh: MaterialMesh2dBundle<ColorMaterial>,
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
}

pub mod plugins {
    use super::systems;
    use bevy::prelude::*;
    use bevy_xpbd_2d::resources::Gravity;

    pub struct ScenePlugin;

    impl Plugin for ScenePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(
                Startup,
                (
                    systems::spawn_camera,
                    systems::spawn_ball,
                    systems::spawn_paddles,
                    systems::spawn_walls,
                    systems::kick_off,
                )
                    .chain(),
            )
            .insert_resource(Gravity::ZERO);
        }
    }
}
