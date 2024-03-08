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

    #[derive(Bundle)]
    struct WallBundle {
        wall: Wall,
        rigid_body: RigidBody,
        collider: Collider,
        restitution: Restitution,
        friction: Friction,
        material_mesh: MaterialMesh2dBundle<ColorMaterial>,
    }

    impl WallBundle {
        fn new(
            meshes: &mut ResMut<Assets<Mesh>>,
            materials: &mut ResMut<Assets<ColorMaterial>>,
            position: (f32, f32, f32),
            size: (f32, f32),
        ) -> Self {
            Self {
                wall: Wall,
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
            (-550.0, 0.0, 0.0),
            (25.0, 200.0),
        ),));

        commands.spawn((PaddleBundle::new(
            &mut meshes,
            &mut materials,
            (550.0, 0.0, 0.0),
            (25.0, 200.0),
        ),));
    }

    pub fn spawn_walls(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((WallBundle::new(
            &mut meshes,
            &mut materials,
            (0.0, 375.0, 0.0),
            (1280.0, 25.0),
        ),));

        commands.spawn((WallBundle::new(
            &mut meshes,
            &mut materials,
            (0.0, -375.0, 0.0),
            (1280.0, 25.0),
        ),));
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
