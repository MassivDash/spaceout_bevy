use bevy::prelude::*;

#[derive(Component)]
pub struct Spaceship {
    pub throttle: f32,
    pub fuel: f32,
    pub hull: f32,
    pub shields: f32,
    pub weapons: u32,
}

pub fn spawn_spaceship(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    _meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // Use the F5S1.png image as the spaceship sprite
    let texture_handle = asset_server.load("F5S1.png");
    commands.spawn((
        Sprite {
            image: texture_handle,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Spaceship {
            throttle: 0.0,
            fuel: 1.0,
            hull: 1.0,
            shields: 1.0,
            weapons: 1,
        },
    ));
}
