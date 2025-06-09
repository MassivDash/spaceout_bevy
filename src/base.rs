use bevy::prelude::*;

#[derive(Component)]
pub struct Base;

pub fn spawn_base(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut base_mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        Default::default(),
    );
    let base_radius = 40.0;
    let segments = 32;
    let mut positions = vec![[0.0, 0.0, 0.0]];
    let mut uvs = vec![[0.5, 0.5]];
    for i in 0..=segments {
        let theta = (i as f32) * std::f32::consts::TAU / (segments as f32);
        positions.push([base_radius * theta.cos(), base_radius * theta.sin(), 0.0]);
        uvs.push([0.5 + 0.5 * theta.cos(), 0.5 + 0.5 * theta.sin()]);
    }
    base_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    base_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; segments + 2]);
    base_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    let mut indices = Vec::new();
    for i in 1..=segments {
        indices.push(0);
        indices.push(i as u32);
        indices.push((i + 1) as u32);
    }
    base_mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    let base = Mesh2d(meshes.add(base_mesh));
    commands.spawn((
        base,
        bevy::sprite::MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 1.0))),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        Base,
    ));
}
