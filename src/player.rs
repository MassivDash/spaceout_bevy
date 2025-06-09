use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub throttle: f32,
}

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    _meshes: &mut ResMut<Assets<Mesh>>,
    _materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // Use the F5S1.png image as the player sprite
    let texture_handle = asset_server.load("F5S1.png");
    commands.spawn((
        Sprite {
            image: texture_handle,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Player { throttle: 0.0 },
    ));
}
