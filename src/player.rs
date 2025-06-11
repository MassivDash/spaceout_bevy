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
    let texture_handle = asset_server.load("s2.png");
    // Set scale to resize sprite to 128px width (adjust 1.0 if your sprite's native width is different)
    let scale_x = 128.0 / 500.0; // Replace 48.0 with your sprite's actual pixel width if different
    commands.spawn((
        Sprite {
            image: texture_handle,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_z(std::f32::consts::PI), // Rotate to face upwards
            scale: Vec3::new(scale_x, scale_x, 1.0),               // Uniform scalings
        },
        Spaceship {
            throttle: 0.0,
            fuel: 150.0,
            hull: 1.0,
            shields: 1.0,
            weapons: 1,
        },
    ));
}
