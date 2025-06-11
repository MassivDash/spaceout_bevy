use bevy::prelude::*;

#[derive(Component)]
pub struct Moon;

pub fn spawn_moon(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let texture_handle = asset_server.load("moon.png");
    commands.spawn((
        Sprite {
            image: texture_handle,
            ..default()
        },
        Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(4400.0, 4300.0, -1.0)),
        Moon,
    ));
}
