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

pub mod commands {
    use crate::prelude::*;

    use bevy::prelude::*;
    use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

    #[derive(Bundle)]
    struct BallBundle {
        ball: Ball,
        rigid_body: RigidBody,
        collider: Collider,
        restitution: Restitution,
        friction: Friction,
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
                rigid_body: RigidBody::Kinematic,
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
        commands.spawn((BallBundle {
            material_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(20.0))),
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            collider: Collider::circle(20.0),
            ..default()
        },));
    }

    pub fn spawn_paddles(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((PaddleBundle {
            material_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(25.0, 200.0))),
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(-550.0, 0.0, 0.0),
                ..default()
            },
            collider: Collider::rectangle(25.0, 200.0),
            ..default()
        },));

        commands.spawn((PaddleBundle {
            material_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(25.0, 200.0))),
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(550.0, 0.0, 0.0),
                ..default()
            },
            collider: Collider::rectangle(25.0, 200.0),
            ..default()
        },));
    }

    pub fn spawn_walls(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((WallBundle {
            material_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(1280.0, 25.0))),
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(0.0, 375.0, 0.0),
                ..default()
            },
            collider: Collider::rectangle(1280.0, 25.0),
            ..default()
        },));

        commands.spawn((WallBundle {
            material_mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(1280.0, 25.0))),
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(0.0, -375.0, 0.0),
                ..default()
            },
            collider: Collider::rectangle(1280.0, 25.0),
            ..default()
        },));
    }
}

pub mod plugins {
    use super::commands;

    use bevy::prelude::*;
    use bevy_xpbd_2d::resources::Gravity;

    pub struct ScenePlugin;

    impl Plugin for ScenePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(
                Startup,
                (
                    commands::spawn_camera,
                    commands::spawn_ball,
                    commands::spawn_paddles,
                    commands::spawn_walls,
                ),
            )
            .insert_resource(Gravity::ZERO);
        }
    }
}
