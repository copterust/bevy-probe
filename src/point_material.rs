use bevy::asset::RenderAssetUsages;
use bevy::mesh::PrimitiveTopology;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

#[derive(Asset, AsBindGroup, Debug, Clone, TypePath)]
pub struct PointMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}

impl Material for PointMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/point.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Opaque
    }
}

/// Build a single mesh containing a small octahedron at each sample position.
fn points_mesh(points: &[Vec3], radius: f32) -> Mesh {
    let offsets: [Vec3; 6] = [
        Vec3::new(radius, 0.0, 0.0),
        Vec3::new(-radius, 0.0, 0.0),
        Vec3::new(0.0, radius, 0.0),
        Vec3::new(0.0, -radius, 0.0),
        Vec3::new(0.0, 0.0, radius),
        Vec3::new(0.0, 0.0, -radius),
    ];

    // 8 faces of octahedron, CCW winding
    let faces: [[usize; 3]; 8] = [
        [2, 4, 0], [2, 0, 5], [2, 5, 1], [2, 1, 4],
        [3, 0, 4], [3, 5, 0], [3, 1, 5], [3, 4, 1],
    ];

    let verts_per_point = faces.len() * 3;
    let total_verts = points.len() * verts_per_point;

    let mut positions = Vec::with_capacity(total_verts);
    let mut normals = Vec::with_capacity(total_verts);

    for &center in points {
        for face in &faces {
            let a = center + offsets[face[0]];
            let b = center + offsets[face[1]];
            let c = center + offsets[face[2]];
            let n = (b - a).cross(c - a).normalize();
            positions.push(a);
            positions.push(b);
            positions.push(c);
            normals.push(n);
            normals.push(n);
            normals.push(n);
        }
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh
}

pub fn spawn_points(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PointMaterial>>,
) {
    let samples = vec![
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.3, 0.2, 0.9),
        Vec3::new(-0.5, 0.7, 0.5),
    ];

    let mesh = meshes.add(points_mesh(&samples, 0.005));

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(materials.add(PointMaterial {
            color: LinearRgba::new(1.0, 0.2, 0.2, 1.0),
        })),
    ));
}
