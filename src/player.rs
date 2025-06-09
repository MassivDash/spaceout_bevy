use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub throttle: f32,
}

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        Default::default(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 30.0, 0.0], [-26.0, -15.0, 0.0], [26.0, -15.0, 0.0]],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.5, 1.0], [0.0, 0.0], [1.0, 0.0]],
    );
    mesh.insert_indices(bevy::render::mesh::Indices::U32(vec![0, 1, 2]));
    let triangle = Mesh2d(meshes.add(mesh));
    commands.spawn((
        triangle,
        bevy::sprite::MeshMaterial2d(materials.add(Color::srgb(0.2, 0.8, 0.3))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Player { throttle: 0.0 },
    ));
}
