use bevy::prelude::*;

#[derive(Component)]
pub struct Sun;

pub fn spawn_sun(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let texture_handle = asset_server.load("sun.png");
    commands.spawn((
        Sprite {
            image: texture_handle,
            ..default()
        },
        Transform::from_scale(Vec3::splat(8.0)).with_translation(Vec3::new(8000.0, 8000.0, -1.0)),
        Sun,
    ));
}

// System to rotate the sun slowly
pub fn rotate_sun(mut query: Query<&mut Transform, With<Sun>>) {
    for mut transform in &mut query {
        transform.rotate_z(0.01); // Adjust speed as desired
    }
}
