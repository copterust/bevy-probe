use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

mod point_material;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(MaterialPlugin::<point_material::PointMaterial>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, draw_axes)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    points_materials: ResMut<Assets<point_material::PointMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        PanOrbitCamera::default(),
    ));

    // Light
    commands.spawn((DirectionalLight::default(), Transform::default()));

    // Unit sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgba(0.2, 0.2, 0.2, 0.05),
            alpha_mode: AlphaMode::Blend,
            ..default()
        })),
    ));

    point_material::spawn_points(commands, meshes, points_materials);
}

fn draw_axes(mut gizmos: Gizmos) {
    gizmos.axes(Transform::IDENTITY, 1.0);
}
